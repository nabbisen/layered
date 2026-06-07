//! Derived outline index: section nodes, identity, and traversal
//! (RFC-006 node identity and range semantics, RFC-007 tree construction).
//!
//! The outline is **derived data**: it can always be dropped and rebuilt from
//! the canonical source text without losing document content.

use std::collections::HashMap;

use crate::error::IndexError;
use crate::range::ByteRange;

/// Markdown heading depth, H1 through H6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    /// Numeric depth, 1..=6.
    pub fn as_u8(self) -> u8 {
        match self {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }

    /// From numeric depth; `None` outside 1..=6.
    pub fn from_u8(level: u8) -> Option<Self> {
        match level {
            1 => Some(HeadingLevel::H1),
            2 => Some(HeadingLevel::H2),
            3 => Some(HeadingLevel::H3),
            4 => Some(HeadingLevel::H4),
            5 => Some(HeadingLevel::H5),
            6 => Some(HeadingLevel::H6),
            _ => None,
        }
    }
}

/// Identity of a section node within (and, where structure permits, across)
/// indexing revisions.
///
/// IDs are a deterministic hash of the node's **ordinal path** from the root
/// (the sequence of zero-based child indices), per the adopted RFC-006
/// algorithm. Consequences:
///
/// - duplicate heading titles receive distinct IDs (different paths);
/// - body-only edits keep IDs stable (structure unchanged, paths unchanged);
/// - IDs are deterministic for identical document structure.
///
/// IDs are **not** guaranteed to survive edits that change heading structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

impl NodeId {
    /// Deterministic FNV-1a hash of an ordinal path. Dependency-free and
    /// stable across Rust versions, unlike `std`'s default hasher.
    pub(crate) fn from_ordinal_path(path: &[usize]) -> NodeId {
        const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
        const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
        let mut hash = FNV_OFFSET;
        // Length first so `[]` (root) and `[0]` cannot alias trivially.
        for byte in (path.len() as u64).to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        for ordinal in path {
            for byte in (*ordinal as u64).to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        }
        NodeId(hash)
    }
}

/// One navigable section: the synthetic root or a Markdown heading section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionNode {
    pub id: NodeId,
    /// `None` only for the synthetic root.
    pub parent_id: Option<NodeId>,
    /// `None` only for the synthetic root (RFC-007 root design).
    pub level: Option<HeadingLevel>,
    /// Plain-text heading title (inline markup flattened). Empty for root.
    pub title: String,
    /// The heading line(s) including the trailing newline; empty at byte 0
    /// for the root.
    pub heading_range: ByteRange,
    /// Section body: after the heading line(s), up to the first child heading
    /// or the end of `full_range`. Excludes child sections (RFC-006).
    pub body_range: ByteRange,
    /// Heading start through the byte before the next heading of the same or
    /// shallower level, or end of document.
    pub full_range: ByteRange,
    /// Children in source order.
    pub children: Vec<NodeId>,
    /// Zero-based index among siblings.
    pub ordinal: usize,
}

impl SectionNode {
    /// Whether this is the synthetic root node.
    pub fn is_root(&self) -> bool {
        self.level.is_none()
    }
}

/// The derived heading tree over the canonical source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outline {
    root_id: NodeId,
    /// All nodes; index 0 is the root, headings follow in source order.
    nodes: Vec<SectionNode>,
    by_id: HashMap<NodeId, usize>,
}

impl Outline {
    pub(crate) fn from_nodes(nodes: Vec<SectionNode>) -> Result<Self, IndexError> {
        let root_id = nodes.first().filter(|n| n.is_root()).map(|n| n.id).ok_or(
            IndexError::InvariantViolation("outline must start with a root node"),
        )?;
        let mut by_id = HashMap::with_capacity(nodes.len());
        for (idx, node) in nodes.iter().enumerate() {
            if by_id.insert(node.id, idx).is_some() {
                return Err(IndexError::InvariantViolation("duplicate node id"));
            }
        }
        Ok(Self {
            root_id,
            nodes,
            by_id,
        })
    }

    /// Identity of the synthetic root node.
    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    /// The synthetic root node.
    pub fn root(&self) -> &SectionNode {
        &self.nodes[0]
    }

    /// Looks up a node by ID.
    pub fn node(&self, id: NodeId) -> Option<&SectionNode> {
        self.by_id.get(&id).map(|&idx| &self.nodes[idx])
    }

    /// Whether the outline contains `id`.
    pub fn contains(&self, id: NodeId) -> bool {
        self.by_id.contains_key(&id)
    }

    /// All nodes in source order, root first.
    pub fn iter(&self) -> impl Iterator<Item = &SectionNode> {
        self.nodes.iter()
    }

    /// Number of nodes including the root.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// True when the document has no headings (root only).
    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 1
    }

    /// Direct children of `id`, in source order.
    pub fn children(&self, id: NodeId) -> Option<Vec<&SectionNode>> {
        let node = self.node(id)?;
        Some(node.children.iter().filter_map(|c| self.node(*c)).collect())
    }

    /// Path from the root to `id`, inclusive on both ends.
    pub fn path(&self, id: NodeId) -> Option<Vec<&SectionNode>> {
        let mut rev = Vec::new();
        let mut cursor = self.node(id)?;
        loop {
            rev.push(cursor);
            match cursor.parent_id {
                Some(parent) => cursor = self.node(parent)?,
                None => break,
            }
        }
        rev.reverse();
        Some(rev)
    }

    /// Validates the RFC-006 invariants against the source text. Failure
    /// indicates an indexer bug, never invalid user Markdown.
    pub fn validate(&self, source: &str) -> Result<(), IndexError> {
        for node in &self.nodes {
            for range in [&node.heading_range, &node.body_range, &node.full_range] {
                if range.validate_in(source).is_err() {
                    return Err(IndexError::InvariantViolation(
                        "range invalid against source",
                    ));
                }
            }
            if node.full_range.start > node.heading_range.start
                || node.heading_range.end > node.body_range.start
                || node.body_range.end > node.full_range.end
            {
                return Err(IndexError::InvariantViolation("range ordering broken"));
            }
            if node.is_root() {
                if node.parent_id.is_some() {
                    return Err(IndexError::InvariantViolation("root must have no parent"));
                }
            } else {
                let parent_id = node.parent_id.ok_or(IndexError::InvariantViolation(
                    "non-root node without parent",
                ))?;
                let parent = self
                    .node(parent_id)
                    .ok_or(IndexError::InvariantViolation("parent id unresolved"))?;
                if !parent.full_range.contains_range(&node.full_range) {
                    return Err(IndexError::InvariantViolation(
                        "child full range escapes parent full range",
                    ));
                }
                if parent.children.get(node.ordinal) != Some(&node.id) {
                    return Err(IndexError::InvariantViolation("ordinal/children mismatch"));
                }
            }
            for child in &node.children {
                if !self.contains(*child) {
                    return Err(IndexError::InvariantViolation("child id unresolved"));
                }
            }
        }
        Ok(())
    }
}
