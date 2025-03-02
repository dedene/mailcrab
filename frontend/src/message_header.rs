use crate::types::MailMessage;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Eq, PartialEq)]
pub struct MessageHeaderProps {
    pub message: MailMessage,
}

#[function_component(MessageHeader)]
pub fn view(props: &MessageHeaderProps) -> Html {
    let message = &props.message;

    if message.id.is_empty() {
        return html! {};
    }

    html! {
      <>
        <table>
          <tbody>
            <tr>
              <th>{"From"}</th>
              <td>
                <span class="name">
                  {message.from.clone().name.unwrap_or_default()}
                </span>
                <span class="email">
                  {message.from.clone().email.unwrap_or_default()}
                </span>
              </td>
            </tr>
            <tr>
              <th>{"To"}</th>
              <td>
                {message.to.iter().map(|to| {
                  html! {
                    <span class="user">
                      <span class="name">
                        {to.clone().name.unwrap_or_default()}
                      </span>
                      <span class="email">
                        {to.clone().email.unwrap_or_default()}
                      </span>
                    </span>
                  }
                }).collect::<Html>()}
              </td>
            </tr>
            <tr>
              <th>{"Subject"}</th>
              <td>{&message.subject}</td>
            </tr>
          </tbody>
        </table>
        <div class="attachments">
          {message.attachments.iter().map(|a| {
            html! {
              <a
                href={format!("data:{};base64,{}", &a.mime, &a.content)}
                download={a.filename.clone()}
                class={&a.mime.replace('/', "-")}
              >
                {&a.filename}
                <span class="size">{&a.size}</span>
              </a>
            }
          }).collect::<Html>()}
        </div>
      </>
    }
}
