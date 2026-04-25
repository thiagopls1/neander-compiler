use crate::frontend::parser::{Expr, Operator, Program, Statement};

pub fn generate(program: &Program) -> String {
    let mut data_section = String::new();
    let mut text_section = String::new();

    data_section.push_str(".DATA\n");
    text_section.push_str(".TEXT\n.ORG 100\n");

    for stmt in &program.statements {
        match stmt {
            Statement::VarDeclare { name, value } => {
                // Inicialização simples
                match value {
                    Expr::Number(n) => {
                        data_section.push_str(&format!("\t{} DATA {}\n", name, n));
                    }
                    _ => {
                        // Expressão → inicializa com 0 e calcula depois
                        data_section.push_str(&format!("\t{} DATA 0\n", name));

                        generate_expr(value, &mut text_section);
                        text_section.push_str(&format!("\tSTA [{}]\n", name));
                    }
                }
            }
            Statement::VarAssign { name, value } => {
                generate_expr(value, &mut text_section);
                text_section.push_str(&format!("\tSTA [{}]\n", name));
            }
        }
    }

    text_section.push_str("\tHLT\n");

    format!("{}\n{}", data_section, text_section)
}

fn generate_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Number(n) => {
            out.push_str(&format!("\tLDA {}\n", n));
        }

        Expr::Variable(name) => {
            out.push_str(&format!("\tLDA [{}]\n", name));
        }

        Expr::Operation { left, op, right } => {
            generate_expr(left, out);
            // Faz o "deref", pois está envolvido em um Box
            match &**right {
                Expr::Number(number) => match op {
                    Operator::Add => out.push_str(&format!("\tADD {}\n", number)),
                    Operator::Sub => {
                        out.push_str(&format!("\tNOT\n"));
                        out.push_str(&format!("\tADD {}\n", number));
                    }
                },

                Expr::Variable(name) => match op {
                    Operator::Add => out.push_str(&format!("\tADD [{}]\n", name)),
                    Operator::Sub => {
                        out.push_str(&format!("\tNOT\n"));
                        out.push_str(&format!("\tADD {}\n", name));
                    }
                },
                _ => {
                    generate_expr(right, out);
                }
            }
        }
    }
}
