use crate::dbt_common::clients::jinja_blocks::{BlockIterator, BlockTag, TagIterator};

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
    warning_callback: Option<&dyn Fn(/*ExtractWarning*/)>
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
    let block_iterator = BlockIterator::new(tag_iterator, warning_callback);
    let blocks = block_iterator.lex_for_blocks(allowed_blocks, Some(collect_raw_data));
    blocks

    // if test_caching_enabled() {
    //     let hash = _get_blocks_hash(text, allowed_blocks, collect_raw_data);
    //     _TESTING_BLOCKS_CACHE.lock().unwrap().insert(hash, blocks.clone());
    // }

}

