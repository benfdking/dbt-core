use crate::clients::jinja_blocks::{BlockIterator, BlockTag, TagIterator};

/// Extract the top-level blocks with matching block types from a jinja file.
///
/// Includes some special handling for block nesting.
///
/// # Arguments
/// * `text` - The data to extract blocks from.
/// * `allowed_blocks` - The names of the blocks to extract from the file. They may not be nested within if/for blocks. If None, use the default values.
/// * `collect_raw_data` - If set, raw data between matched blocks will also be part of the results, as `BlockData` objects. They have a `block_type_name` field of `'__dbt_data'` and will never have a `block_name`.
/// * `warning_callback` - An optional callback that will be called if there are recoverable issues detected in the template.
///
/// # Returns
/// A vector of `BlockTag`s matching the allowed block types and (if `collect_raw_data` is `true`) `BlockData` objects.
pub fn extract_top_level_blocks(
    text: &str,
    allowed_blocks: Option<&std::collections::HashSet<String>>,
    collect_raw_data: bool,
    warning_callback: Option<&dyn Fn(&str)>,
) -> Vec<BlockTag> {
    // If test caching is enabled, check the cache first
    // if test_caching_enabled() {
    //     let hash = _get_blocks_hash(text, allowed_blocks, collect_raw_data);
    //     if let Some(cached) = _TESTING_BLOCKS_CACHE.lock().unwrap().get(&hash) {
    //         // Return a clone of the cached result
    //         return cached.clone();
    //     }
    // }

    let tag_iterator = TagIterator::new(text.to_string());
    let mut block_iterator = BlockIterator::new(tag_iterator, warning_callback);
    let blocks = block_iterator.lex_for_blocks(allowed_blocks, Some(collect_raw_data));
    blocks

    // if test_caching_enabled() {
    //     let hash = _get_blocks_hash(text, allowed_blocks, collect_raw_data);
    //     _TESTING_BLOCKS_CACHE.lock().unwrap().insert(hash, blocks.clone());
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BAR_BLOCK: &str = r#"{% mytype bar %}
{# a comment
    that inside it has
    {% mytype baz %}
{% endmyothertype %}
{% endmytype %}
{% endmytype %}
    {#
{% endmytype %}#}

some other stuff

{%- endmytype%}"#;

    const X_BLOCK: &str = r#"
{% myothertype x %}
before
{##}
and after
{% endmyothertype %}
"#;

    #[test]
    fn test_basic() {
        let body = "{{ config(foo=\"bar\") }}\r\nselect * from this.that\r\n";
        let block_data = format!("  \n\r\t{{%- mytype foo %}}{}{{% endmytype -%}}", body);
        
        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        
        let blocks = extract_top_level_blocks(
            &block_data,
            Some(&allowed_blocks),
            false,
            None
        );
        
        assert_eq!(blocks.len(), 1);
        let b0 = &blocks[0];
        assert_eq!(b0.block_type_name, "mytype");
        assert_eq!(b0.block_name, "foo");
        assert_eq!(b0.contents, Some(body.to_string()));
        assert_eq!(b0.full_block, Some(block_data.to_string()));
    }

    #[test]
    fn test_multiple() {
        let body_one = "{{ config(foo=\"bar\") }}\r\nselect * from this.that\r\n";
        let body_two = "{{ config(bar=1)}}\r\nselect * from {% if foo %} thing {% else %} other_thing {% endif %}";

        let block_data = format!(
            "  {{%- mytype foo %}}{}{{% endmytype -%}}\r\n{{%- othertype bar %}}{}{{% endothertype -%}}",
            body_one,
            body_two
        );

        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        allowed_blocks.insert("othertype".to_string());

        let blocks = extract_top_level_blocks(
            &block_data,
            Some(&allowed_blocks),
            false,
            None
        );

        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_comments() {
        let body = "{{ config(foo=\"bar\") }}\r\nselect * from this.that\r\n";
        let comment = "{# my comment #}";
        let block_data = "  \n\r\t{%- mytype foo %}".to_string() + body + "{%endmytype -%}";
        
        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        
        let blocks = extract_top_level_blocks(
            &(comment.to_string() + &block_data),
            Some(&allowed_blocks),
            false,
            None
        );
        
        assert_eq!(blocks.len(), 1);
        let b0 = &blocks[0];
        assert_eq!(b0.block_type_name, "mytype");
        assert_eq!(b0.block_name, "foo");
        assert_eq!(b0.contents, Some(body.to_string()));
        assert_eq!(b0.full_block, Some(block_data));
    }

    #[test]
    fn test_evil_comments() {
        let body = "{{ config(foo=\"bar\") }}\r\nselect * from this.that\r\n";
        let comment = "{# external comment {% othertype bar %} select * from thing.other_thing{% endothertype %} #}";
        let block_data = "  \n\r\t{%- mytype foo %}".to_string() + body + "{%endmytype -%}";
        
        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        
        let blocks = extract_top_level_blocks(
            &(comment.to_string() + &block_data),
            Some(&allowed_blocks),
            false,
            None
        );
        
        assert_eq!(blocks.len(), 1);
        let b0 = &blocks[0];
        assert_eq!(b0.block_type_name, "mytype");
        assert_eq!(b0.block_name, "foo");
        assert_eq!(b0.contents, Some(body.to_string()));
        assert_eq!(b0.full_block, Some(block_data));
    }

    #[test]
    fn test_nested_comments() {
        let body = "{# my comment #} {{ config(foo=\"bar\") }}\r\nselect * from {# my other comment embedding {% endmytype %} #} this.that\r\n";
        let block_data = "  \n\r\t{%- mytype foo %}".to_string() + body + "{% endmytype -%}";
        let comment = "{# external comment {% othertype bar %} select * from thing.other_thing{% endothertype %} #}";
        
        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        
        let blocks = extract_top_level_blocks(
            &(comment.to_string() + &block_data),
            Some(&allowed_blocks),
            false,
            None
        );
        
        assert_eq!(blocks.len(), 1);
        let b0 = &blocks[0];
        assert_eq!(b0.block_type_name, "mytype");
        assert_eq!(b0.block_name, "foo");
        assert_eq!(b0.contents, Some(body.to_string()));
        assert_eq!(b0.full_block, Some(block_data));
    }

    #[test]
    fn test_complex_file() {
        let complex_snapshot_file = format!(
            r#"
{{#some stuff {{% mytype foo %}} #}}
{{% mytype foo %}} some stuff {{% endmytype %}}

{}{}"#,
            BAR_BLOCK,
            X_BLOCK
        );
        
        let mut allowed_blocks = std::collections::HashSet::new();
        allowed_blocks.insert("mytype".to_string());
        allowed_blocks.insert("myothertype".to_string());
        
        let blocks = extract_top_level_blocks(
            &complex_snapshot_file,
            Some(&allowed_blocks),
            false,
            None
        );
        
        assert_eq!(blocks.len(), 3);
        
        let b0 = &blocks[0];
        assert_eq!(b0.block_type_name, "mytype");
        assert_eq!(b0.block_name, "foo");
        assert_eq!(b0.full_block, Some("{% mytype foo %} some stuff {% endmytype %}".to_string()));
        assert_eq!(b0.contents, Some(" some stuff ".to_string()));

        let b1 = &blocks[1];
        assert_eq!(b1.block_type_name, "mytype");
        assert_eq!(b1.block_name, "bar");
        assert_eq!(b1.full_block, Some(BAR_BLOCK.to_string()));
        assert_eq!(b1.contents, Some(BAR_BLOCK[16..BAR_BLOCK.len()-15].trim_end().to_string()));

        let b2 = &blocks[2];
        assert_eq!(b2.block_type_name, "myothertype");
        assert_eq!(b2.block_name, "x");
        assert_eq!(b2.full_block, Some(X_BLOCK.trim().to_string()));
        assert_eq!(
            b2.contents,
            Some(X_BLOCK["\n{% myothertype x %}".len()..X_BLOCK.len()-"{% endmyothertype %}\n".len()].to_string())
        );
    }
}