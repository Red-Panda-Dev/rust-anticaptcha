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

pub enum ControlEnpPostfix {
    // get account info
    getBalance,
    getQueueStats,
    getAppStats,
    getSpendingStats,
    // reports
    reportIncorrectImageCaptcha,
    reportIncorrectRecaptcha,
    reportCorrectRecaptcha,
    reportIncorrectHcaptcha,
    pushAntiGateVariable,
    // additional
    test,
}

impl ControlEnpPostfix {
    pub fn value_as_string(&self) -> String {
        match &self {
            ControlEnpPostfix::getBalance => "getBalance".to_string(),
            ControlEnpPostfix::getQueueStats => "getQueueStats".to_string(),
            ControlEnpPostfix::getAppStats => "getAppStats".to_string(),
            ControlEnpPostfix::getSpendingStats => "getSpendingStats".to_string(),
            ControlEnpPostfix::reportIncorrectImageCaptcha => {
                "reportIncorrectImageCaptcha".to_string()
            }
            ControlEnpPostfix::reportIncorrectRecaptcha => "reportIncorrectRecaptcha".to_string(),
            ControlEnpPostfix::reportCorrectRecaptcha => "reportCorrectRecaptcha".to_string(),
            ControlEnpPostfix::reportIncorrectHcaptcha => "reportIncorrectHcaptcha".to_string(),
            ControlEnpPostfix::pushAntiGateVariable => "pushAntiGateVariable".to_string(),
            ControlEnpPostfix::test => "test".to_string(),
        }
    }
}
