// Copyright The KCL Authors. All rights reserved.

// Auto generated, DONOT EDIT!!!

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ApiType {
    Value,
}

impl std::fmt::Display for ApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiType::Value => write!(f, "{:?}", "api::kclvm::Value"),
        }
    }
}

impl ApiType {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        format!("{self:?}")
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ApiFunc {
    kclvm_assert,
    kclvm_base64_decode,
    kclvm_base64_encode,
    kclvm_builtin_abs,
    kclvm_builtin_all_true,
    kclvm_builtin_any_true,
    kclvm_builtin_bin,
    kclvm_builtin_bool,
    kclvm_builtin_dict,
    kclvm_builtin_float,
    kclvm_builtin_hex,
    kclvm_builtin_int,
    kclvm_builtin_isnullish,
    kclvm_builtin_isunique,
    kclvm_builtin_len,
    kclvm_builtin_list,
    kclvm_builtin_max,
    kclvm_builtin_min,
    kclvm_builtin_multiplyof,
    kclvm_builtin_oct,
    kclvm_builtin_option,
    kclvm_builtin_option_init,
    kclvm_builtin_option_reset,
    kclvm_builtin_ord,
    kclvm_builtin_pow,
    kclvm_builtin_print,
    kclvm_builtin_range,
    kclvm_builtin_round,
    kclvm_builtin_sorted,
    kclvm_builtin_str,
    kclvm_builtin_str_capitalize,
    kclvm_builtin_str_chars,
    kclvm_builtin_str_count,
    kclvm_builtin_str_endswith,
    kclvm_builtin_str_find,
    kclvm_builtin_str_format,
    kclvm_builtin_str_index,
    kclvm_builtin_str_isalnum,
    kclvm_builtin_str_isalpha,
    kclvm_builtin_str_isdigit,
    kclvm_builtin_str_islower,
    kclvm_builtin_str_isspace,
    kclvm_builtin_str_istitle,
    kclvm_builtin_str_isupper,
    kclvm_builtin_str_join,
    kclvm_builtin_str_lower,
    kclvm_builtin_str_lstrip,
    kclvm_builtin_str_removeprefix,
    kclvm_builtin_str_removesuffix,
    kclvm_builtin_str_replace,
    kclvm_builtin_str_rfind,
    kclvm_builtin_str_rindex,
    kclvm_builtin_str_rsplit,
    kclvm_builtin_str_rstrip,
    kclvm_builtin_str_split,
    kclvm_builtin_str_splitlines,
    kclvm_builtin_str_startswith,
    kclvm_builtin_str_strip,
    kclvm_builtin_str_title,
    kclvm_builtin_str_upper,
    kclvm_builtin_sum,
    kclvm_builtin_typeof,
    kclvm_builtin_zip,
    kclvm_config_attr_map,
    kclvm_context_delete,
    kclvm_context_invoke,
    kclvm_context_new,
    kclvm_context_pkgpath_is_imported,
    kclvm_context_set_debug_mode,
    kclvm_context_set_disable_none,
    kclvm_context_set_disable_schema_check,
    kclvm_context_set_import_names,
    kclvm_context_set_kcl_filename,
    kclvm_context_set_kcl_line_col,
    kclvm_context_set_kcl_location,
    kclvm_context_set_kcl_modpath,
    kclvm_context_set_kcl_pkgpath,
    kclvm_context_set_kcl_workdir,
    kclvm_context_set_strict_range_check,
    kclvm_convert_collection_value,
    kclvm_crypto_blake3,
    kclvm_crypto_fileblake3,
    kclvm_crypto_filesha256,
    kclvm_crypto_filesha512,
    kclvm_crypto_md5,
    kclvm_crypto_sha1,
    kclvm_crypto_sha224,
    kclvm_crypto_sha256,
    kclvm_crypto_sha384,
    kclvm_crypto_sha512,
    kclvm_crypto_uuid,
    kclvm_datetime_date,
    kclvm_datetime_now,
    kclvm_datetime_ticks,
    kclvm_datetime_today,
    kclvm_datetime_validate,
    kclvm_default_collection_insert_int_pointer,
    kclvm_default_collection_insert_value,
    kclvm_dict_clear,
    kclvm_dict_get,
    kclvm_dict_get_entry,
    kclvm_dict_get_value,
    kclvm_dict_get_value_by_path,
    kclvm_dict_has_value,
    kclvm_dict_insert,
    kclvm_dict_insert_unpack,
    kclvm_dict_insert_value,
    kclvm_dict_is_override_attr,
    kclvm_dict_keys,
    kclvm_dict_len,
    kclvm_dict_merge,
    kclvm_dict_remove,
    kclvm_dict_safe_insert,
    kclvm_dict_set_value,
    kclvm_dict_update,
    kclvm_dict_update_key_value,
    kclvm_dict_values,
    kclvm_file_abs,
    kclvm_file_append,
    kclvm_file_cp,
    kclvm_file_current,
    kclvm_file_delete,
    kclvm_file_exists,
    kclvm_file_glob,
    kclvm_file_mkdir,
    kclvm_file_modpath,
    kclvm_file_mv,
    kclvm_file_read,
    kclvm_file_read_env,
    kclvm_file_size,
    kclvm_file_workdir,
    kclvm_file_write,
    kclvm_iterator_cur_key,
    kclvm_iterator_cur_value,
    kclvm_iterator_delete,
    kclvm_iterator_is_end,
    kclvm_iterator_next_value,
    kclvm_json_decode,
    kclvm_json_dump_to_file,
    kclvm_json_encode,
    kclvm_json_validate,
    kclvm_list_append,
    kclvm_list_append_bool,
    kclvm_list_append_float,
    kclvm_list_append_int,
    kclvm_list_append_str,
    kclvm_list_append_unpack,
    kclvm_list_clear,
    kclvm_list_count,
    kclvm_list_find,
    kclvm_list_get,
    kclvm_list_get_option,
    kclvm_list_insert,
    kclvm_list_len,
    kclvm_list_pop,
    kclvm_list_pop_first,
    kclvm_list_remove_at,
    kclvm_list_resize,
    kclvm_list_set,
    kclvm_manifests_yaml_stream,
    kclvm_math_ceil,
    kclvm_math_exp,
    kclvm_math_expm1,
    kclvm_math_factorial,
    kclvm_math_floor,
    kclvm_math_gcd,
    kclvm_math_isfinite,
    kclvm_math_isinf,
    kclvm_math_isnan,
    kclvm_math_log,
    kclvm_math_log10,
    kclvm_math_log1p,
    kclvm_math_log2,
    kclvm_math_modf,
    kclvm_math_pow,
    kclvm_math_sqrt,
    kclvm_net_IP_string,
    kclvm_net_fqdn,
    kclvm_net_is_IP,
    kclvm_net_is_IPv4,
    kclvm_net_is_global_unicast_IP,
    kclvm_net_is_interface_local_multicast_IP,
    kclvm_net_is_link_local_multicast_IP,
    kclvm_net_is_link_local_unicast_IP,
    kclvm_net_is_loopback_IP,
    kclvm_net_is_multicast_IP,
    kclvm_net_is_unspecified_IP,
    kclvm_net_join_host_port,
    kclvm_net_parse_IP,
    kclvm_net_split_host_port,
    kclvm_net_to_IP16,
    kclvm_net_to_IP4,
    kclvm_plugin_init,
    kclvm_plugin_invoke,
    kclvm_plugin_invoke_json,
    kclvm_regex_compile,
    kclvm_regex_findall,
    kclvm_regex_match,
    kclvm_regex_replace,
    kclvm_regex_search,
    kclvm_regex_split,
    kclvm_runtime_catch,
    kclvm_schema_assert,
    kclvm_schema_backtrack_cache,
    kclvm_schema_default_settings,
    kclvm_schema_do_check_with_index_sign_attr,
    kclvm_schema_get_value,
    kclvm_schema_instances,
    kclvm_schema_optional_check,
    kclvm_schema_value_check,
    kclvm_schema_value_new,
    kclvm_scope_add_setter,
    kclvm_scope_delete,
    kclvm_scope_get,
    kclvm_scope_new,
    kclvm_scope_set,
    kclvm_template_execute,
    kclvm_template_html_escape,
    kclvm_units_to_G,
    kclvm_units_to_Gi,
    kclvm_units_to_K,
    kclvm_units_to_Ki,
    kclvm_units_to_M,
    kclvm_units_to_Mi,
    kclvm_units_to_P,
    kclvm_units_to_Pi,
    kclvm_units_to_T,
    kclvm_units_to_Ti,
    kclvm_units_to_m,
    kclvm_units_to_n,
    kclvm_units_to_u,
    kclvm_value_Bool,
    kclvm_value_Decorator,
    kclvm_value_Dict,
    kclvm_value_False,
    kclvm_value_Float,
    kclvm_value_Function,
    kclvm_value_Function_using_ptr,
    kclvm_value_Int,
    kclvm_value_List,
    kclvm_value_List10,
    kclvm_value_List6,
    kclvm_value_None,
    kclvm_value_Schema,
    kclvm_value_Str,
    kclvm_value_Str_ptr,
    kclvm_value_True,
    kclvm_value_Undefined,
    kclvm_value_Unit,
    kclvm_value_as,
    kclvm_value_check_function_ptr,
    kclvm_value_cmp_equal_to,
    kclvm_value_cmp_greater_than,
    kclvm_value_cmp_greater_than_or_equal,
    kclvm_value_cmp_less_than,
    kclvm_value_cmp_less_than_or_equal,
    kclvm_value_cmp_not_equal_to,
    kclvm_value_deep_copy,
    kclvm_value_delete,
    kclvm_value_from_json,
    kclvm_value_function_invoke,
    kclvm_value_function_ptr,
    kclvm_value_in,
    kclvm_value_is,
    kclvm_value_is_not,
    kclvm_value_is_truthy,
    kclvm_value_iter,
    kclvm_value_len,
    kclvm_value_load_attr,
    kclvm_value_load_attr_option,
    kclvm_value_logic_and,
    kclvm_value_logic_or,
    kclvm_value_not_in,
    kclvm_value_op_add,
    kclvm_value_op_aug_add,
    kclvm_value_op_aug_bit_and,
    kclvm_value_op_aug_bit_lshift,
    kclvm_value_op_aug_bit_or,
    kclvm_value_op_aug_bit_rshift,
    kclvm_value_op_aug_bit_xor,
    kclvm_value_op_aug_div,
    kclvm_value_op_aug_floor_div,
    kclvm_value_op_aug_mod,
    kclvm_value_op_aug_mul,
    kclvm_value_op_aug_pow,
    kclvm_value_op_aug_sub,
    kclvm_value_op_bit_and,
    kclvm_value_op_bit_lshift,
    kclvm_value_op_bit_or,
    kclvm_value_op_bit_rshift,
    kclvm_value_op_bit_xor,
    kclvm_value_op_div,
    kclvm_value_op_floor_div,
    kclvm_value_op_mod,
    kclvm_value_op_mul,
    kclvm_value_op_pow,
    kclvm_value_op_sub,
    kclvm_value_plan_to_json,
    kclvm_value_plan_to_yaml,
    kclvm_value_remove_item,
    kclvm_value_schema_function,
    kclvm_value_schema_with_config,
    kclvm_value_slice,
    kclvm_value_slice_option,
    kclvm_value_subscr,
    kclvm_value_subscr_option,
    kclvm_value_subscr_set,
    kclvm_value_to_json_value,
    kclvm_value_to_json_value_with_null,
    kclvm_value_to_str_value,
    kclvm_value_to_yaml_value,
    kclvm_value_unary_l_not,
    kclvm_value_unary_minus,
    kclvm_value_unary_not,
    kclvm_value_unary_plus,
    kclvm_value_union,
    kclvm_value_union_all,
    kclvm_yaml_decode,
    kclvm_yaml_decode_all,
    kclvm_yaml_dump_all_to_file,
    kclvm_yaml_dump_to_file,
    kclvm_yaml_encode,
    kclvm_yaml_encode_all,
    kclvm_yaml_validate,
}

impl std::fmt::Display for ApiFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ApiFunc {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        format!("{self:?}")
    }
}
