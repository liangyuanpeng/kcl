use std::vec;

use kclvm_error::Position;
use kclvm_sema::core::{
    global_state::GlobalState,
    symbol::{KCLSymbol, SymbolKind, SymbolRef},
};
use kclvm_sema::ty::TypeKind;
use lsp_types::{SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensResult};

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRUCT,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::TYPE,
    SemanticTokenType::MACRO,
    SemanticTokenType::COMMENT,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::FUNCTION,
];

pub(crate) struct KCLSemanticToken {
    pub start: Position,
    pub kind: u32,
    pub length: u32,
}

pub fn semantic_tokens_full(file: &str, gs: &GlobalState) -> Option<SemanticTokensResult> {
    let mut kcl_tokens: Vec<KCLSemanticToken> = vec![];
    let sema_db = gs.get_sema_db();
    if let Some(file_sema) = sema_db.get_file_sema(file) {
        let symbols = file_sema.get_symbols();
        for symbol_ref in symbols {
            if let Some(symbol) = gs.get_symbols().get_symbol(*symbol_ref) {
                let (start, end) = symbol.get_range();
                match get_kind(*symbol_ref, symbol, gs) {
                    Some(kind) => {
                        kcl_tokens.push(KCLSemanticToken {
                            start: start.clone(),
                            kind,
                            length: if start.line == end.line {
                                (end.column.unwrap_or(0) - start.column.unwrap_or(0)) as u32
                            } else {
                                symbol.get_name().len() as u32
                            },
                        });
                    }
                    None => continue,
                }
            }
        }
    }

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: kcl_semantic_tokens_to_semantic_tokens(&mut kcl_tokens),
    }))
}

pub(crate) fn get_kind(symbol_ref: SymbolRef, symbol: &KCLSymbol, gs: &GlobalState) -> Option<u32> {
    match symbol_ref.get_kind() {
        SymbolKind::Schema => Some(type_index(SemanticTokenType::STRUCT)),
        SymbolKind::Attribute => Some(type_index(SemanticTokenType::PROPERTY)),
        SymbolKind::Package => Some(type_index(SemanticTokenType::NAMESPACE)),
        SymbolKind::TypeAlias => Some(type_index(SemanticTokenType::TYPE)),
        SymbolKind::Value => {
            if let Some(ty) = &symbol.get_sema_info().ty {
                match ty.kind {
                    TypeKind::Function(_) => Some(type_index(SemanticTokenType::FUNCTION)),
                    _ => Some(type_index(SemanticTokenType::VARIABLE)),
                }
            } else {
                Some(type_index(SemanticTokenType::VARIABLE))
            }
        }
        SymbolKind::Function => Some(type_index(SemanticTokenType::FUNCTION)),
        SymbolKind::Rule => Some(type_index(SemanticTokenType::MACRO)),
        SymbolKind::Unresolved => match &symbol.get_definition() {
            Some(def_ref) => match gs.get_symbols().get_symbol(*def_ref) {
                Some(symbol) => get_kind(*def_ref, symbol, gs),
                None => Some(type_index(SemanticTokenType::VARIABLE)),
            },
            None => {
                let unresolved_symbol = gs.get_symbols().get_unresolved_symbol(symbol_ref).unwrap();
                if unresolved_symbol.is_type() {
                    Some(type_index(SemanticTokenType::TYPE))
                } else {
                    Some(type_index(SemanticTokenType::VARIABLE))
                }
            }
        },
        SymbolKind::Expression => None,
        SymbolKind::Comment => None,
        SymbolKind::Decorator => Some(type_index(SemanticTokenType::PARAMETER)),
    }
}

pub(crate) fn type_index(ty: SemanticTokenType) -> u32 {
    LEGEND_TYPE.iter().position(|it| *it == ty).unwrap() as u32
}

pub(crate) fn kcl_semantic_tokens_to_semantic_tokens(
    tokens: &mut [KCLSemanticToken],
) -> Vec<SemanticToken> {
    tokens.sort_by(|a, b| {
        if a.start.line == b.start.line {
            a.start
                .column
                .unwrap_or(0)
                .cmp(&b.start.column.unwrap_or(0))
        } else {
            a.start.line.cmp(&b.start.line)
        }
    });

    // ref: https://github.com/microsoft/vscode-extension-samples/blob/5ae1f7787122812dcc84e37427ca90af5ee09f14/semantic-tokens-sample/vscode.proposed.d.ts#L71
    // A file can contain many tokens, perhaps even hundreds of thousands of tokens. Therefore, to improve
    // the memory consumption around describing semantic tokens, we have decided to avoid allocating an object
    // for each token and we represent tokens from a file as an array of integers. Furthermore, the position
    // of each token is expressed relative to the token before it because most tokens remain stable relative to
    // each other when edits are made in a file.

    let mut semantic_tokens: Vec<SemanticToken> = vec![];
    if tokens.is_empty() {
        return semantic_tokens;
    }
    let first_token = tokens.first().unwrap();

    semantic_tokens.push(SemanticToken {
        delta_line: (first_token.start.line - 1) as u32,
        delta_start: (first_token.start.column.unwrap_or(0)) as u32,
        length: first_token.length,
        token_type: first_token.kind,
        token_modifiers_bitset: 0,
    });

    for token_tuple in tokens.windows(2) {
        let first_token = &token_tuple[0];
        let second_token = &token_tuple[1];
        let delta_line = (second_token.start.line - first_token.start.line) as u32;
        let delta_start = (if delta_line == 0 {
            second_token.start.column.unwrap_or(0) - first_token.start.column.unwrap_or(0)
        } else {
            second_token.start.column.unwrap_or(0)
        }) as u32;
        if delta_line != 0 || delta_start != 0 {
            semantic_tokens.push(SemanticToken {
                delta_line,
                delta_start,
                length: second_token.length,
                token_type: second_token.kind,
                token_modifiers_bitset: 0,
            });
        }
    }
    semantic_tokens
}

#[cfg(test)]
mod tests {
    use crate::tests::compile_test_file;
    use proc_macro_crate::bench_test;

    use super::semantic_tokens_full;

    #[test]
    #[bench_test]
    fn semantic_tokens_full_test() {
        let (file, _, _, gs, _) = compile_test_file("src/test_data/sema_token/sema_token.k");
        let res = semantic_tokens_full(&file, &gs);
        if let Some(tokens) = res {
            match &tokens {
                lsp_types::SemanticTokensResult::Tokens(tokens) => {
                    let get: Vec<(u32, u32, u32, u32)> = tokens
                        .data
                        .iter()
                        .map(|token| {
                            (
                                token.delta_line,
                                token.delta_start,
                                token.length,
                                token.token_type,
                            )
                        })
                        .collect();
                    // (delta line, delta col(if delta line != 0, from 0), length, kind)
                    // (0, 15, 1, 3), // m
                    // (1, 5, 3, 4),  // num
                    // (1, 7, 7, 1),  // Persons
                    // (1, 4, 4, 2),  // name
                    // (2, 0, 2, 0),  // p5
                    // (0, 4, 7, 1),  // Persons
                    // (0, 10, 7, 1), // Persons
                    // (1, 4, 4, 2),  // name
                    // (2, 0, 1, 0),  // n
                    // (0, 3, 3, 4),  // num
                    // (2, 0, 4, 8),  // func
                    // (0, 14, 1, 0), // x
                    // (1, 4, 1, 0),  // x
                    // (3, 0, 1, 0),  // a
                    // (0, 4, 4, 8),  // func
                    // (1, 0, 1, 0),  // b
                    // (0, 4, 4, 8),  // func
                    // (0, 5, 1, 0)   // x
                    // (2, 7, 8, 1)   // Manifest
                    // (1, 5, 4, 0)   // name
                    // (2, 0, 3, 0)   // aaa
                    // (0, 5, 3, 4)   // any
                    // (2, 0, 3, 0)   // bbb
                    // (0, 10, 4, 0)  // item
                    // (2, 7, 3, 3), // net
                    // (2, 0, 1, 0), // c
                    // (1, 4, 3, 0)  // net
                    insta::assert_snapshot!(format!("{:?}", get));
                }
                lsp_types::SemanticTokensResult::Partial(_) => {
                    panic!("test failed")
                }
            }
        }
    }
}
