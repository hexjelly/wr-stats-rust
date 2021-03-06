use Targets;
use DataRow;
use elma::Time;

pub fn create_html_table(data: &[DataRow], targets_table: &[Targets]) -> String {
    let headers = vec![
        "Level".to_string(),
        "PR".to_string(),
        "Target".to_string(),
        "Diff".to_string(),
        "Kuski to beat".to_string(),
        "Time beat".to_string(),
        "Kuski beat".to_string(),
    ];
    let mut html_table = String::new();
    html_table.push_str(&inline_tr(&table_header(&headers)));

    for (i, r) in data.iter().enumerate() {
        html_table.push_str(&table_data_s(&format!(
            "{}. {}",
            &r.lev_number.to_string(),
            &r.lev_name
        )));
        html_table.push_str(&time_to_tagged_td(&r.pr, &targets_table[i]));

        if let Some(ref wr) = r.wr_not_beat {
            html_table.push_str(&time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&time_to_diff(&(r.pr - wr.time)));
            html_table.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
        }

        if let Some(ref wr) = r.wr_beat {
            html_table.push_str(&time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
        }

        html_table = inline_tr(&html_table);
    }

    html_table = inline_table(&html_table);

    html_table
}

pub fn create_html(html_table: &str) -> String {
    format!(
        r#"
            <!doctype html>
            <html>
                <head>
                    {styles}
                </head>
                <body>
                    {table}
                </body>
            </html>
            "#,
        styles = inline_style(include_str!("styles.css")),
        table = html_table
    )
}

fn table_header(h: &[String]) -> String {
    h.iter().map(|x| format!("<th>{}</th>", *x)).collect()
}

fn table_data_s(s: &str) -> String {
    format!("<td>{}</td>", s)
}

fn inline_tr(h: &str) -> String {
    format!("<tr>{}</tr>", h)
}

fn table_num(h: &str) -> String {
    format!("(<i>{}</i>)", h)
}

fn inline_table(s: &str) -> String {
    format!(r#"<table>{}</table>"#, s)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn time_to_diff(t: &Time) -> String {
    format!("<td>+{}</td>", t.to_string())
}

fn time_to_tagged_td(t: &Time, tar: &Targets) -> String {
    let class = match *t {
        t if t > tar.beginner => "unclassified",
        t if t > tar.ok => "beginner",
        t if t > tar.good => "ok",
        t if t > tar.professional => "good",
        t if t > tar.world_class => "professional",
        t if t > tar.legendary => "world_class",
        t if t > tar.godlike => "legendary",
        _ => "godlike",
    };
    format!("<td class=\"{}\">{}</td>", class, t.to_string())
}
