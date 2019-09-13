// 企業情報
pub struct DomainCompany {
    id: i32,
    name: String,
    logo: String,
    thumbnail: String,
}

impl DomainCompany {
    pub fn new(id: i32, name: String, logo: String, thumbnail: String) -> DomainCompany {
        DomainCompany {
            id,
            name,
            logo,
            thumbnail,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn logo(&self) -> String {
        self.logo.to_string()
    }
    pub fn thumbnail(&self) -> String {
        self.thumbnail.to_string()
    }
}
