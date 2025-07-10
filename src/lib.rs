use std::sync::Arc;

use harper_core::{linting::{LintGroup, Linter}, parsers, remove_overlaps, Dialect, Document, FstDictionary, MergedDictionary, MutableDictionary};

pub fn grammar_check(input:&str, dialect:Dialect, immutable_dictionary:Option<FstDictionary>, user_dictionary:Option<MutableDictionary>) -> () {
    println!("Starting grammar check.");

    println!("");
    println!("Input is:");
    println!("{}",input.to_string());

    println!("");
    println!("Building dictionary. This is time consuming!");
    let mut merged_dict = MergedDictionary::new();
    merged_dict.add_dictionary(FstDictionary::curated());

    let document = Document::new(input, &parsers::PlainEnglish, &merged_dict);

    match immutable_dictionary{
        Some(dict)=>{merged_dict.add_dictionary(Arc::new(dict));},
        None=>()
    }

    match user_dictionary{
        Some(dict)=>{merged_dict.add_dictionary(Arc::new(dict));},
        None=>()
    }

    println!("Building linter.");
    let mut linter = LintGroup::new_curated(Arc::new(merged_dict), dialect);

    println!("Setting rules.");
    //Lint with all rules
    linter.set_all_rules_to(Some(true));
    
    println!("Linting");
    let mut lints = linter.lint(&document);

    println!("Removing overlaps.");
    remove_overlaps(&mut lints);

    println!("");
    println!("Results:");
    for lint in lints {
        println!("");
        println!("{}, {}, {}-{}",lint.lint_kind,lint.priority,lint.span.start,lint.span.end);
        for suggestion in lint.suggestions
        {
            println!("   {}",suggestion);
        }
    }

    println!("");
    println!("Grammar check finished.");
}