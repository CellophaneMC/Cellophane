use bevy_reflect::Reflect;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Reflect, Default)]
pub struct GameProfile {
    id: Uuid,
    name: String,
}

impl GameProfile {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn offline_uuid(username: &str) -> Uuid {
    let mut context = md5::Context::new();
    context.consume(format!("OfflinePlayer:{}", username).as_bytes());
    let computed = context.compute();
    let bytes = computed.into();

    let mut builder = uuid::Builder::from_bytes(bytes);

    builder
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Md5);

    builder.into_uuid()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}

impl Property {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Self { name, value, signature }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
