use iced::Renderer;
use iced::Theme;
use iced::window::settings;


use iced::widget::{button, column, row, text, Column};

pub fn main() -> iced::Result {
    let settings = iced::window::settings::Settings{
        size: iced::Size::new(350.0, 150.0),
        //position: Position,
        //min_size: Option<Size>,
        //max_size: Option<Size>,
        //visible: bool,
        //resizable: bool,
        //decorations: bool,
        //transparent: bool,
        //level: Level,
        //icon: Option<Icon>,
        //platform_specific: PlatformSpecific,
        //exit_on_close_request: bool,
        ..Default::default()
    };

    iced::run("test",
              Phoma::update,
              Phoma::view)



//    iced::application(
 //       "test",
  //      Phoma::update,
   //     Phoma::view)
    //    //.window(settings)
     //   .run()
}

struct Phoma {
    value: String,
    capitalize: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Display(String),
    Capitalize,
    Remove,
}

impl Phoma {
    fn new() -> Self {
        Phoma {
            value: "".to_string(),
            capitalize: false,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Capitalize => {
                self.capitalize = !self.capitalize;
            }
            Message::Display(content) => {
                self.value = content;
            }
            Message::Remove => {
                self.value = "".to_string();
            }
        }
    }

    fn view(&self) -> Column<Message> {
         let mut a_content = 'a';
        // let mut b_content = 'b';
        // let mut c_content = 'c';
        // let mut d_content = 'd';
        // let mut e_content = 'e';
        // let mut f_content = 'f';
        // let mut g_content = 'g';
        // let mut h_content = 'h';
        // let mut i_content = 'i';
        // let mut j_content = 'j';
        // let mut k_content = 'k';
        // let mut l_content = 'l';
        // let mut m_content = 'm';
        // let mut n_content = 'n';
        // let mut o_content = 'o';
        // let mut p_content = 'p';
        // let mut q_content = 'q';
        // let mut r_content = 'r';
        // let mut s_content = 's';
        // let mut t_content = 't';
        // let mut u_content = 'u';
        // let mut v_content = 'v';
        // let mut w_content = 'w';
        // let mut x_content = 'x';
        // let mut y_content = 'y';
        // let mut z_content = 'z';

        // if self.capitalize == true {
        //     a_content = 'A';
        //     b_content = 'B';
        //     c_content = 'C';
        //     d_content = 'D';
        //     e_content = 'E';
        //     f_content = 'F';
        //     g_content = 'G';
        //     h_content = 'H';
        //     i_content = 'I';
        //     j_content = 'J';
        //     k_content = 'K';
        //     l_content = 'L';
        //     m_content = 'M';
        //     n_content = 'N';
        //     o_content = 'O';
        //     p_content = 'P';
        //     q_content = 'Q';
        //     r_content = 'R';
        //     s_content = 'S';
        //     t_content = 'T';
        //     u_content = 'U';
        //     v_content = 'V';
        //     w_content = 'W';
        //     x_content = 'X';
        //     y_content = 'Y';
        //     z_content = 'Z';
        // }

        let a_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(a_content).center())
             .width(30)
             .height(35);
        // let b_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(b_content).center())
        //     .width(30)
        //     .height(35);
        // let c_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(c_content).center())
        //     .width(30)
        //     .height(35);
        // let d_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(d_content).center())
        //     .width(30)
        //     .height(35);
        // let e_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(e_content).center())
        //     .width(30)
        //     .height(35);
        // let f_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(f_content).center())
        //     .width(30)
        //     .height(35);
        // let g_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(g_content).center())
        //     .width(30)
        //     .height(35);
        // let h_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(h_content).center())
        //     .width(30)
        //     .height(35);
        // let i_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(i_content).center())
        //     .width(30)
        //     .height(35);
        // let j_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(j_content).center())
        //     .width(30)
        //     .height(35);
        // let k_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(k_content).center())
        //     .width(30)
        //     .height(35);
        // let l_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(l_content).center())
        //     .width(30)
        //     .height(35);
        // let m_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(m_content).center())
        //     .width(30)
        //     .height(35);
        // let n_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(n_content).center())
        //     .width(30)
        //     .height(35);
        // let o_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(o_content).center())
        //     .width(30)
        //     .height(35);
        // let p_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(p_content).center())
        //     .width(30)
        //     .height(35);
        // let q_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(q_content).center())
        //     .width(30)
        //     .height(35);
        // let r_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(r_content).center())
        //     .width(30)
        //     .height(35);
        // let s_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(s_content).center())
        //     .width(30)
        //     .height(35);
        // let t_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(t_content).center())
        //     .width(30)
        //     .height(35);
        // let u_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(u_content).center())
        //     .width(30)
        //     .height(35);
        // let v_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(v_content).center())
        //     .width(30)
        //     .height(35);
        // let w_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(w_content).center())
        //     .width(30)
        //     .height(35);
        // let x_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(x_content).center())
        //     .width(30)
        //     .height(35);
        // let y_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(y_content).center())
        //     .width(30)
        //     .height(35);
        // let z_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text(z_content).center())
        //     .width(30)
        //     .height(35);

        // let capitalize_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text("⇑").shaping(text::Shaping::Advanced).center())
        //     .width(30)
        //     .height(35);
        // let remove_button: iced::widget::Button<'_, Message, Theme, Renderer>  = iced::widget::button(text("←").shaping(text::Shaping::Advanced).center())
        //     .width(30)
        //     .height(35);

         column![
        //     text(self.value.clone()),
        //     row![
        //         q_button.on_press(Message::Display(q_content.to_string())),
        //         w_button.on_press(Message::Display(w_content.to_string())),
        //         e_button.on_press(Message::Display(e_content.to_string())),
        //         r_button.on_press(Message::Display(r_content.to_string())),
        //         t_button.on_press(Message::Display(t_content.to_string())),
        //         z_button.on_press(Message::Display(z_content.to_string())),
        //         u_button.on_press(Message::Display(u_content.to_string())),
        //         i_button.on_press(Message::Display(i_content.to_string())),
        //         o_button.on_press(Message::Display(o_content.to_string())),
        //         p_button.on_press(Message::Display(p_content.to_string())),
        //     ].spacing(5),
        //     row![
        //         a_button.on_press(Message::Display(a_content.to_string())),
        //         s_button.on_press(Message::Display(s_content.to_string())),
        //         d_button.on_press(Message::Display(d_content.to_string())),
        //         f_button.on_press(Message::Display(f_content.to_string())),
        //         g_button.on_press(Message::Display(g_content.to_string())),
        //         h_button.on_press(Message::Display(h_content.to_string())),
        //         j_button.on_press(Message::Display(j_content.to_string())),
        //         k_button.on_press(Message::Display(k_content.to_string())),
        //         l_button.on_press(Message::Display(l_content.to_string())),
        //     ].spacing(5),
        //     row![
        //         capitalize_button.on_press(Message::Capitalize),
        //         y_button.on_press(Message::Display(y_content.to_string())),
        //         x_button.on_press(Message::Display(x_content.to_string())),
        //         c_button.on_press(Message::Display(c_content.to_string())),
        //         v_button.on_press(Message::Display(v_content.to_string())),
        //         b_button.on_press(Message::Display(b_content.to_string())),
        //         n_button.on_press(Message::Display(n_content.to_string())),
        //         m_button.on_press(Message::Display(m_content.to_string())),
        //         remove_button.on_press(Message::Remove),
        //     ].spacing(5)
         ].spacing(5).align_x(iced::alignment::Horizontal::Center)
    }
}


impl Default for Phoma {
    fn default() -> Self {
        Phoma::new()
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use iced_test::selector::text;
//     use iced_test::{Error, simulator};

//     #[test]
//     fn it_counts() -> Result<(), Error> {
//         let mut counter = Counter { value: 0 };
//         let mut ui = simulator(counter.view());

//         let _ = ui.click(text("Display"))?;
//         let _ = ui.click(text("Display"))?;
//         let _ = ui.click(text("Decrement"))?;

//         for message in ui.into_messages() {
//             counter.update(message);
//         }

//         assert_eq!(counter.value, 1);

//         let mut ui = simulator(counter.view());
//         assert!(ui.find(text("1")).is_ok(), "Counter should display 1!");

//         Ok(())
//     }
// }
