use crate::{document, Document, DocumentId};

struct DocumentSlot(DocumentId, Document);

pub struct Collection {
    documents: Vec<DocumentSlot>,
    next_id: u32,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            documents: Vec::new(),
            next_id: 1,
        }
    }

    pub fn document(&self, id: DocumentId) -> Option<&Document> {
        self.find_document_index(id)
            .ok()
            .map(|id| &self.documents[id].1)
    }

    pub fn insert(&mut self, doc: Document) -> DocumentId {
        let id = self.next_id.into();
        self.next_id += 1;
        self.documents.push(DocumentSlot(id, doc));
        id
    }

    pub fn delete(&mut self, id: DocumentId) -> bool {
        match self.find_document_index(id) {
            Ok(idx) => {
                self.documents.remove(idx);
                true
            }
            Err(_) => false,
        }
    }

    fn find_document_index(&self, id: DocumentId) -> Result<usize, usize> {
        self.documents
            .binary_search_by_key(&id, |DocumentSlot(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Collection, Document, Field, FieldValue};

    use super::DocumentSlot;

    #[test]
    pub fn inserting_doc_generates_next_id() {
        let mut col = Collection::new();
        let id = col.insert(Document::new());
        assert_eq!(1, id.value());
    }

    #[test]
    pub fn removing_doc_does_not_allow_reusing_id() {
        let mut col = Collection::new();
        let id = col.insert(Document::new());
        col.delete(id);
        let id = col.insert(Document::new());
        assert_eq!(2, id.value());
    }

    #[test]
    pub fn can_remove_document_from_middle_without_disrupting_sorting() {
        let mut col = Collection::new();
        col.insert(Document::new());
        col.insert(Document::new());
        let id = col.insert(Document::new());
        col.insert(Document::new());

        col.delete(id);

        assert_eq!(
            vec![1, 2, 4],
            col.documents
                .iter()
                .map(|d| d.0.value())
                .collect::<Vec<u32>>()
        );
    }

    #[test]
    pub fn can_retrieve_document_by_id() {
        fn create_test_doc(val: &'static str) -> Document {
            let mut doc = Document::new();
            doc.add_field(Field::new("test", val));
            doc
        }

        let mut col = Collection::new();
        col.insert(create_test_doc("doc1"));
        col.insert(create_test_doc("doc2"));
        let id = col.insert(create_test_doc("doc3"));
        col.insert(create_test_doc("doc4"));
        col.delete(id);

        assert_eq!(
            &FieldValue::Str("doc1".into()),
            col.document(1.into()).unwrap().field_value("test").unwrap()
        );
        assert_eq!(
            &FieldValue::Str("doc2".into()),
            col.document(2.into()).unwrap().field_value("test").unwrap()
        );
        assert_eq!(
            &FieldValue::Str("doc4".into()),
            col.document(4.into()).unwrap().field_value("test").unwrap()
        );
    }
}
