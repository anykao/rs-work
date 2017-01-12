#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debut)]
struct SGMailV3 {
    From             *Email,
        Subject          String ,
        Personalizations []*Personalization,
        Content          []*Content,
        Attachments      []*Attachment,
        TemplateID       string,
        Sections         map[string]string,  `json:"sections,omitempty"`
        Headers          map[string]string,  `json:"headers,omitempty"`
        Categories       []string,           `json:"categories,omitempty"`
        CustomArgs       map[string]string,  `json:"custom_args,omitempty"`
        SendAt           int,                `json:"send_at,omitempty"`
        BatchID          string,             `json:"batch_id,omitempty"`
        Asm              *Asm,               `json:"asm,omitempty"`
        IPPoolID         string,             `json:"ip_pool_name,omitempty"`
        MailSettings     *MailSettings,      `json:"mail_settings,omitempty"`
        TrackingSettings *TrackingSettings,  `json:"tracking_settings,omitempty"`
        ReplyTo          *Email,             `json:"reply_to,omitempty"`
}

#[derive(Serialize, Deserialize)]
struct Personalization struct {
    To            []*Email          `json:"to,omitempty"`
        CC            []*Email          `json:"cc,omitempty"`
        BCC           []*Email          `json:"bcc,omitempty"`
        Subject       string            `json:"subject,omitempty"`
        Headers       map[string]string `json:"headers,omitempty"`
        Substitutions map[string]string `json:"substitutions,omitempty"`
        CustomArgs    map[string]string `json:"custom_args,omitempty"`
        Categories    []string          `json:"categories,omitempty"`
        SendAt        int               `json:"send_at,omitempty"`
}

struct Email  {
    Name    String `json:"name,omitempty"`
        Address String `json:"email,omitempty"`
}

struct Content  {
    Type  string `json:"type,omitempty"`
        Value string `json:"value,omitempty"`
}
