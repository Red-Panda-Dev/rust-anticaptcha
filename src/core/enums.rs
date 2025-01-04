pub trait TaskTypeTrait {
    fn value_as_string(&self) -> String;
}

pub enum ImageTaskType {
    ImageToTextTask,
    ImageToCoordinatesTask,
}
impl TaskTypeTrait for ImageTaskType {
    fn value_as_string(&self) -> String {
        match &self {
            ImageTaskType::ImageToTextTask => "ImageToTextTask".to_string(),
            ImageTaskType::ImageToCoordinatesTask => "ImageToCoordinatesTask".to_string(),
        }
    }
}
pub enum TokenTaskType {
    RecaptchaV2Task,
    RecaptchaV2TaskProxyless,

    RecaptchaV3Enterprise,
    RecaptchaV3TaskProxyless,

    RecaptchaV2EnterpriseTask,
    RecaptchaV2EnterpriseTaskProxyless,

    FunCaptchaTask,
    FunCaptchaTaskProxyless,

    GeeTestTask,
    GeeTestTaskProxyless,

    TurnstileTask,
    TurnstileTaskProxyless,

    AntiGateTask,
}

impl TaskTypeTrait for TokenTaskType {
    fn value_as_string(&self) -> String {
        match &self {
            TokenTaskType::RecaptchaV2Task => "RecaptchaV2Task".to_string(),
            TokenTaskType::RecaptchaV2TaskProxyless => "RecaptchaV2TaskProxyless".to_string(),
            TokenTaskType::RecaptchaV3Enterprise => "RecaptchaV3Enterprise".to_string(),
            TokenTaskType::RecaptchaV3TaskProxyless => "RecaptchaV3TaskProxyless".to_string(),
            TokenTaskType::RecaptchaV2EnterpriseTask => "RecaptchaV2EnterpriseTask".to_string(),
            TokenTaskType::RecaptchaV2EnterpriseTaskProxyless => {
                "RecaptchaV2EnterpriseTaskProxyless".to_string()
            }
            TokenTaskType::FunCaptchaTask => "FunCaptchaTask".to_string(),
            TokenTaskType::FunCaptchaTaskProxyless => "FunCaptchaTaskProxyless".to_string(),
            TokenTaskType::GeeTestTask => "GeeTestTask".to_string(),
            TokenTaskType::GeeTestTaskProxyless => "GeeTestTaskProxyless".to_string(),
            TokenTaskType::TurnstileTask => "TurnstileTask".to_string(),
            TokenTaskType::TurnstileTaskProxyless => "TurnstileTaskProxyless".to_string(),
            TokenTaskType::AntiGateTask => "AntiGateTask".to_string(),
        }
    }
}

pub enum EnpPostfix {
    // tasks processing
    createTask,
    getTaskResult,
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
}

impl EnpPostfix {
    pub fn value_as_string(&self) -> String {
        match &self {
            EnpPostfix::createTask => "createTask".to_string(),
            EnpPostfix::getTaskResult => "getTaskResult".to_string(),
            EnpPostfix::getBalance => "getBalance".to_string(),
            EnpPostfix::getQueueStats => "getQueueStats".to_string(),
            EnpPostfix::getAppStats => "getAppStats".to_string(),
            EnpPostfix::getSpendingStats => "getSpendingStats".to_string(),
            EnpPostfix::reportIncorrectImageCaptcha => "reportIncorrectImageCaptcha".to_string(),
            EnpPostfix::reportIncorrectRecaptcha => "reportIncorrectRecaptcha".to_string(),
            EnpPostfix::reportCorrectRecaptcha => "reportCorrectRecaptcha".to_string(),
            EnpPostfix::reportIncorrectHcaptcha => "reportIncorrectHcaptcha".to_string(),
            EnpPostfix::pushAntiGateVariable => "pushAntiGateVariable".to_string(),
        }
    }
}

pub enum GetResultStatus {
    processing,
    ready,
    error,
}

impl GetResultStatus {
    pub fn value_as_string(&self) -> String {
        match &self {
            GetResultStatus::processing => "processing".to_string(),
            GetResultStatus::ready => "ready".to_string(),
            GetResultStatus::error => "error".to_string(),
        }
    }
}
