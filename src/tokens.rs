// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[derive(Copy, Clone)]
pub enum SolfegeTokens {
    Do      = 0,
    DoSharp = 1,
    Di,
    Re,
    ReSharp,
    Ri,
    Mi,
    MiSharp,
    Fa,
    FaSharp,
    Fi,
    Sol,
    SolSharp,
    Si,
    SiSharp,
    La,
    LaSharp,
    Li,
    Ti,
    // Do
    // Ti
    Te,
    // La
    Le,
    // Sol
    Se,
    // Fa
    // Mi
    Me,
    // Re
    Ra,

    Undefined
}

pub struct InitTokens {
    pub        generated_data: String,
    pub        print_data    : String,
    pub        put_data      : String,

    pub(crate) tokens        : std::collections::HashMap<String, SolfegeTokens>,

    pub(crate) is_statement  : bool,
    pub(crate) is_print      : bool,
    pub(crate) is_put        : bool
}

impl InitTokens {
    pub fn to(&self, data: &str) -> String {
        data.to_string()
    }

    pub fn init_tokens(&mut self, is_bf: bool) {
        if !is_bf {
            self.generated_data = self.to(
                "/* lol */\n\
                                 #include <stdio.h>\n\
                                 #include <stdlib.h>\n\
                                 \n\n\
                                 int main(int argc, char** argv) {\n\
                                 unsigned char* ptr = calloc(30000, 1);\n");
        }
        
        self.add_token("Do", SolfegeTokens::Do);
        self.add_token("Do#", SolfegeTokens::DoSharp);
        self.add_token("Di", SolfegeTokens::Di);
        self.add_token("Re", SolfegeTokens::Re);
        self.add_token("Re#", SolfegeTokens::ReSharp);
        self.add_token("Ri", SolfegeTokens::Ri);
        self.add_token("Mi", SolfegeTokens::Mi);
        self.add_token("Mi#", SolfegeTokens::MiSharp);
        self.add_token("Fa", SolfegeTokens::Fa);
        self.add_token("Fa#", SolfegeTokens::FaSharp);
        self.add_token("Fi", SolfegeTokens::Fi);
        self.add_token("Sol", SolfegeTokens::Sol);
        self.add_token("Sol#", SolfegeTokens::SolSharp);
        self.add_token("Si", SolfegeTokens::Si);
        self.add_token("Si#", SolfegeTokens::SiSharp);
        self.add_token("La", SolfegeTokens::La);
        self.add_token("La#", SolfegeTokens::LaSharp);
        self.add_token("Li", SolfegeTokens::Li);
        self.add_token("Ti", SolfegeTokens::Ti);
        self.add_token("Te", SolfegeTokens::Te);
        self.add_token("Le", SolfegeTokens::Le);
        self.add_token("Se", SolfegeTokens::Se);
        self.add_token("Me", SolfegeTokens::Me);
        self.add_token("Ra", SolfegeTokens::Ra);
        self.add_token("Lol", SolfegeTokens::Undefined);

        self.is_statement = false;
        self.is_print     = false;
        self.is_put       = false;
    }

    pub fn tokenize(&self, token: &String) -> &SolfegeTokens {
        let get_type = self.tokens.get(token);

        if get_type.is_none() { return &SolfegeTokens::Undefined; }

        get_type.unwrap()
    }

    pub fn codegen(&mut self, re: SolfegeTokens, ti: String) {
        match re {
            SolfegeTokens::Do => {
                self.generated_data.push_str("++ptr;\n")
            },
            SolfegeTokens::DoSharp => {
                if self.is_statement {
                    self.generated_data.push_str("*ptr ");
                }
            },
            SolfegeTokens::Di => {
                self.generated_data.push_str(";\n")
            },
            SolfegeTokens::Re => {
                self.generated_data.push_str("--ptr;\n")
            },
            SolfegeTokens::ReSharp => {
                self.generated_data.push_str("}\n")
            },
            SolfegeTokens::Ri => {
                self.generated_data.push_str("*ti = 0;\n")
            },
            SolfegeTokens::Mi => {
                self.generated_data.push_str("++*ptr;\n")
            },
            SolfegeTokens::MiSharp => {
                self.generated_data.push_str("{\n\n")
            },
            SolfegeTokens::Fa => {
                self.generated_data.push_str("--*ptr;\n")
            },
            SolfegeTokens::FaSharp => {
                self.generated_data.push('(')
            },
            SolfegeTokens::Fi => {
                self.generated_data.push_str("printf(\"%s\", \"");

                self.is_print = true;
            },
            SolfegeTokens::Sol => {
                self.generated_data.push_str("putchar(*ptr);\n")
            },
            SolfegeTokens::SolSharp => {
                self.generated_data.push(')')
            },
            SolfegeTokens::Si => {
                self.generated_data.push_str(&format!("{}\");\n", self.print_data));

                self.is_print = false;
                self.print_data.clear();
            },
            SolfegeTokens::SiSharp => {
                // Si# ... La#
                self.is_put = true
            },
            SolfegeTokens::La => {
                self.generated_data.push_str("*ptr = getchar();\n")
            },
            SolfegeTokens::LaSharp => {
                self.generated_data.push_str(&self.put_data);

                self.put_data.clear();
            },
            SolfegeTokens::Li => {
                 if self.is_statement {
                     self.generated_data.push_str("}\n");
                     self.is_statement = false;

                     return;
                 }

                self.is_statement = true;
                self.generated_data.push_str("if");
            },
            SolfegeTokens::Ti => {
                self.generated_data.push_str("break;\n")
            },
            SolfegeTokens::Te => {
                if self.is_statement {
                    self.generated_data.push('=');
                }
            },
            SolfegeTokens::Le   => {
                if self.is_statement {
                    self.generated_data.push('>');
                }
            },
            SolfegeTokens::Se => {
                if self.is_statement {
                    self.generated_data.push('<');
                }
            },
            SolfegeTokens::Me => {
                if self.is_statement {
                    self.generated_data.push('!');
                }
            },
            SolfegeTokens::Ra => {
                self.is_statement = true;

                self.generated_data.push_str("while");
            },
            _ => {
                if self.is_print {
                    self.print_data.push_str(&ti);
                    self.print_data.push(' ');
                }
            }
        }
    }

    pub fn bf_to_solfege(&mut self, re: char) {
        match re {
            '>' => {
                self.generated_data.push_str("Do ")
            },
            '<' => {
                self.generated_data.push_str("Re ")
            },
            '+' => {
                self.generated_data.push_str("Mi ")
            },
            '-' => {
                self.generated_data.push_str("Fa ")
            },
            '.' => {
                self.generated_data.push_str("Sol ")
            },
            ',' => {
                self.generated_data.push_str("La ")
            },
            '[' => {
                self.generated_data.push_str("Ra Fa# Do# Sol# Mi#\n")
            },
            ']' => {
                self.generated_data.push_str("Re# ");
            }
            _ => {}
        }
    }

    pub fn add_token(&mut self, token: &str, __type: SolfegeTokens) {
        self.tokens.insert(self.to(token), __type);
    }
}