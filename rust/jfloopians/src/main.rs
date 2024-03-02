fn main() {
    let sokol = NewsPaper {
    headline: String::from("Hristo Botev Prevze Radecki!"),
        content: String::from("Uspq da vzeme Radecki!"),
        date: String::from("1 - 04 - 1876"),
    };

    let sokol_sum = sokol.summerize();

    println!("{}", sokol_sum);
    
    let ihlon = Tweet {
        user: String::from("ihlon ma"),
        content: String::from("birds arent real istg"),
        online_status: true,
    };

    let ihlon_sum = ihlon.summerize();

    println!("{ihlon_sum}");

}

trait Summary {
    fn summerize(&self) -> String;
}

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub online_status: bool, 
}

impl Summary for Tweet {
    fn summerize(&self) -> String {
        if self.online_status {
            return format!("{}(onl): {}", self.user, self.content);
        }
            format!("{}(off): {}", self.user, self.content)
    }
}

struct NewsPaper {
    headline: String,
    content: String,
    date: String,
}


impl Summary for NewsPaper {
    fn summerize(&self) -> String {
        format!("{} {}: {}", self.date, self.headline, self.content)
    }
}

