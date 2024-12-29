pub enum TaskType {
    ImageToTextTask,
    RecaptchaV2TaskProxyless,
    RecaptchaV2Task,
}

impl TaskType {
    pub fn value_as_string(&self) -> String {
        match &self {
            TaskType::ImageToTextTask => "ImageToTextTask".to_string(),
            TaskType::RecaptchaV2TaskProxyless => "RecaptchaV2TaskProxyless".to_string(),
            TaskType::RecaptchaV2Task => "RecaptchaV2Task".to_string(),
        }
    }
}
