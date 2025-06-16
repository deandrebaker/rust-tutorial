fn main() {
    {
        let mut s = String::new();

        println!("Empty string: '{}'", s);
    }

    {
        let data = "initial contents";
        let s = data.to_string();
        println!("String from data: '{}'", s);
    }

    {
        let s = String::from("initial contents");
        println!("String from String::from: '{}'", s);
    }

    {
        let hello = String::from("السلام عليكم");
        println!("Different languages: '{}'", hello);
        let hello = String::from("Dobrý den");
        println!("Different languages: '{}'", hello);
        let hello = String::from("Hello");
        println!("Different languages: '{}'", hello);
        let hello = String::from("שלום");
        println!("Different languages: '{}'", hello);
        let hello = String::from("नमस्ते");
        println!("Different languages: '{}'", hello);
        let hello = String::from("こんにちは");
        println!("Different languages: '{}'", hello);
        let hello = String::from("안녕하세요");
        println!("Different languages: '{}'", hello);
        let hello = String::from("你好");
        println!("Different languages: '{}'", hello);
        let hello = String::from("Olá");
        println!("Different languages: '{}'", hello);
        let hello = String::from("Здравствуйте");
        println!("Different languages: '{}'", hello);
        let hello = String::from("Hola");
        println!("Different languages: '{}'", hello);
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("String after push_str: '{}'", s);
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("String after push: '{}'", s);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;

        println!("Concatenated string: '{}'", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;

        println!("Concatenated with multiple strings: '{}'", s);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{s1}-{s2}-{s3}");

        println!("Concatenated with multiple strings: '{}'", s);
    }

    { // Code doesn't compile
        // let hello = "Здравствуйте";
        // let answer = &hello[0];
    }

    {
        let hello = "Здравствуйте";
        let answer = &hello[0..4];
        println!("First four bytes of '{}': '{}'", hello, answer);
    }

    for c in "नमस्ते".chars() {
        println!("Character: '{}'", c);
    }

    for c in "नमस्ते".bytes() {
        println!("Character: '{}'", c);
    }
}
