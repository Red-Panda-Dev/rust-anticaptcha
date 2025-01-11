use std::fmt;

pub trait TaskTypeTrait {
    /// `TaskTypeTrait` analog for `to_string()`
    fn as_string(&self) -> String;
}

/// `ImageTaskType` stored image captcha types
#[derive(Debug)]
pub enum ImageTaskType {
    ImageToTextTask,
    ImageToCoordinatesTask,
}
impl TaskTypeTrait for ImageTaskType {
    /// `ImageTaskType` analog for `to_string()`
    fn as_string(&self) -> String {
        format!("{:?}", &self)
    }
}

/// `TokenTaskType` stored token captcha types
/// Token captcha - captcha solved by tokens
#[derive(Debug)]
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
    /// `TokenTaskType` analog for `to_string()`
    fn as_string(&self) -> String {
        format!("{:?}", &self)
    }
}

/// `EnpPostfix` stored endpoints for service communication
#[derive(Debug)]
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
impl fmt::Display for EnpPostfix {
    /// `EnpPostfix` formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// `GetResultStatus` stored available tasks status
#[derive(Debug)]
pub enum GetResultStatus {
    processing, // captcha still processing
    ready,      // captcha ready
    error,      // captcha solving failed
}
impl fmt::Display for GetResultStatus {
    /// `GetResultStatus` formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
