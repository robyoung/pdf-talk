use lopdf::{dictionary, Document, ObjectId};

pub(crate) trait DocumentAdditions {
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId;
}

impl DocumentAdditions for Document {
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId {
        let catalog_id = self.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        self.trailer.set("Root", catalog_id);
        catalog_id
    }
}
