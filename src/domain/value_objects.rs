use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for MessageId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MessageId> for Uuid {
    fn from(id: MessageId) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DangerScore(f32);

impl DangerScore {
    pub fn new(score: f32) -> Self {
        Self(score.clamp(0.0, 100.0))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn requires_review(&self, threshold: f32) -> bool {
        self.0 > threshold
    }
}

impl From<f32> for DangerScore {
    fn from(score: f32) -> Self {
        Self::new(score)
    }
}

impl From<DangerScore> for f32 {
    fn from(score: DangerScore) -> Self {
        score.0
    }
}
