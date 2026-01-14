type NodeIndex = u32;

#[derive(Default, Clone)]
struct Node {
    /// Sorted vector of children (Char -> NodeIndex).
    /// Kept as a Vec because node degree is usually small in natural language tries.
    children: Vec<(char, NodeIndex)>,
    /// The replacement string if this node marks the end of a valid mapping.
    replacement: Option<Box<str>>,
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        // Check if root has any children
        self.nodes.get(0).map_or(true, |n| n.children.is_empty())
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let mut node_idx = 0;

        // Keys are assumed to be lowercase based on Domain usage
        for c in key.chars() {
            if let Some(idx) = self.find_child_index(node_idx, c) {
                node_idx = idx as usize;
            } else {
                let new_node_idx = self.nodes.len() as NodeIndex;
                self.nodes.push(Node::default());

                let children = &mut self.nodes[node_idx].children;
                children.push((c, new_node_idx));

                // Maintenance: Keep children sorted for Binary Search validity
                children.sort_unstable_by_key(|(k, _)| *k);

                node_idx = new_node_idx as usize;
            }
        }
        self.nodes[node_idx].replacement = Some(value.to_string().into_boxed_str());
    }

    /// Finds the longest matching prefix in `text`.
    /// Returns: Option<(matched_bytes_len, replacement)>
    /// This is a "Zero-Copy" read operation returning a reference to the internal replacement.
    #[must_use]
    pub fn find_longest_prefix(&self, text: &str) -> Option<(usize, &str)> {
        let mut node_idx = 0;
        let mut longest_match = None;
        let mut current_byte_len = 0;

        for c in text.chars() {
            // "Fast & Dirty" case-folding for 1-to-1 mappings.
            // Sufficient for the specific Uzbek Cyrillic/Latin domain constraints.
            let lower_c = c.to_lowercase().next().unwrap_or(c);

            if let Some(next_idx) = self.find_child_index(node_idx, lower_c) {
                node_idx = next_idx as usize;
                current_byte_len += c.len_utf8();

                if let Some(ref replacement) = self.nodes[node_idx].replacement {
                    longest_match = Some((current_byte_len, &**replacement));
                }
            } else {
                break;
            }
        }
        longest_match
    }

    // OPTIMIZATION: Hybrid Search Strategy
    #[inline]
    fn find_child_index(&self, parent_idx: usize, char_to_find: char) -> Option<NodeIndex> {
        let children = &self.nodes[parent_idx].children;

        // Threshold tuning:
        // For small lists (<16), CPU cache locality makes linear scan faster
        // than the branching overhead of binary search.
        if children.len() < 16 {
            return children
                .iter()
                .find(|(c, _)| *c == char_to_find)
                .map(|(_, idx)| *idx);
        }

        // For larger nodes (like the Root), use Binary Search (O(log N)).
        if let Ok(idx) = children.binary_search_by_key(&char_to_find, |(c, _)| *c) {
            Some(children[idx].1)
        } else {
            None
        }
    }
}
