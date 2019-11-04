

///Welcom
pub fn hello()  {
    use ansi_term::Colour;
    println!(r#" {:} "#,Colour::Blue.paint(" "));
    println!(r#" {:} "#,Colour::Blue.paint("██╗██████╗ ██████╗ ██╗ ██████╗  █████╗ ████████╗██████╗  ██████╗ ███╗   ██╗"));
    println!(r#" {:} "#,Colour::Blue.paint("██║██╔══██╗██╔══██╗██║██╔════╝ ██╔══██╗╚══██╔══╝██╔══██╗██╔═══██╗████╗  ██║"));
    println!(r#" {:} "#,Colour::Blue.paint("██║██████╔╝██████╔╝██║██║  ███╗███████║   ██║   ██████╔╝██║   ██║██╔██╗ ██║"));
    println!(r#" {:} "#,Colour::Blue.paint("██║██╔══██╗██╔══██╗██║██║   ██║██╔══██║   ██║   ██╔══██╗██║   ██║██║╚██╗██║"));
    println!(r#" {:} "#,Colour::Blue.paint("██║██║  ██║██║  ██║██║╚██████╔╝██║  ██║   ██║   ██║  ██║╚██████╔╝██║ ╚████║"));
    println!(r#" {:} "#,Colour::Blue.paint("╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝"));
    println!(r#" {:} "#,Colour::Blue.paint("   Der Herr des Wasser und der Pumpe                                       "));
    println!(r#"     "#);
    println!(r#"      "#);

    // println!(  " {:}  ",Colour::Blue.paint(format!("Number of logical cores is {}",num_cpus::get())));

    // println!(r#"  ENVIRONMENTAL MONITORING  "#);
}
