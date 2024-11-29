


pub fn find_similar(a: &str, b: &Vec<String>) -> Vec<String> {
    let mut matches : Vec<String> = Vec::new();
    for item in b {
        let item_prefix = extract_name(&item);
        if let Some(extracted_name) = item_prefix {
            if a.to_lowercase().eq(extracted_name.to_lowercase().as_str()) {
                matches.push(item.clone());
            }
        }
    }
    return matches;
}

fn extract_name(file_name: &str) -> Option<String> {
    let mut name = file_name.to_string();
    let index = name.find('.')?;
    name.truncate(index);
    Some(name)
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::utilities::similar;

    use super::*;

    fn get_string_vec() -> Vec<String> {
        let str_vec = vec![

            "Sass.gitignore",
            "Scala.gitignore",
            "Joomla.gitignore",
            "CakePHP.gitignore",
            "Go.gitignore",
            "Yeoman.gitignore",
            "ZendFramework.gitignore",
            "KiCad.gitignore",
            "Textpattern.gitignore",
            "AL.gitignore",
            "ExpressionEngine.gitignore",
            "Typo3.gitignore",
            "Packer.gitignore",
            "Elisp.gitignore",
            "EPiServer.gitignore",
            "Gcov.gitignore",
            "Haskell.gitignore",
            "GWT.gitignore",
            "LabVIEW.gitignore",
            "C++.gitignore",
            "Objective-C.gitignore",
            "Composer.gitignore",
            "SketchUp.gitignore",
            "Eagle.gitignore",
            "Android.gitignore",
            "Symfony.gitignore",
            "ExtJs.gitignore",
            "Erlang.gitignore",
            "Finale.gitignore",
            "Sdcc.gitignore",
            "LICENSE",
            "Scrivener.gitignore",
            "Qooxdoo.gitignore",
            "RhodesRhomobile.gitignore",
            "PlayFramework.gitignore",
            "Stella.gitignore",
            "SugarCRM.gitignore",
            "Delphi.gitignore",
            "TurboGears2.gitignore",
            "Waf.gitignore",
            "Racket.gitignore",
            "Leiningen.gitignore",
            "Dart.gitignore",
            "R.gitignore",
            "ChefCookbook.gitignore",
            "MetaProgrammingSystem.gitignore",
            "CFWheels.gitignore",
            "Lilypond.gitignore",
            "JENKINS_HOME.gitignore",
            "Processing.gitignore",
            "Kohana.gitignore",
            "Clojure.gitignore",
            "GitBook.gitignore",
            "Lithium.gitignore",
            "Magento.gitignore",
            "Node.gitignore",
            "Python.gitignore",
            "Nim.gitignore",
            "Terraform.gitignore",
            "Yii.gitignore",
            "Nanoc.gitignore",
            "DM.gitignore",
            "Java.gitignore",
            "Elixir.gitignore",
            "WordPress.gitignore",
            "Godot.gitignore",
            "Xojo.gitignore",
            "ArchLinuxPackages.gitignore",
            "Jekyll.gitignore",
            "Ada.gitignore",
            "D.gitignore",
            "README.md",
            "Elm.gitignore",
            "Actionscript.gitignore",
            "Swift.gitignore",
            "Grails.gitignore",
            "Laravel.gitignore",
            "Perl.gitignore",
            "CMake.gitignore",
            "VVVV.gitignore",
            "VisualStudio.gitignore",
            "CraftCMS.gitignore",
            "Coq.gitignore",
            "TwinCAT3.gitignore",
            "Rust.gitignore",
            "Scheme.gitignore",
            "Ballerina.gitignore",
            "CONTRIBUTING.md",
            "IGORPro.gitignore",
            "Drupal.gitignore",
            "Plone.gitignore",
            ".github",
            "GitHubPages.gitignore",
            "AppEngine.gitignore",
            "Mercury.gitignore",
            "JBoss.gitignore",
            "Lua.gitignore",
            "PureScript.gitignore",
            "CUDA.gitignore",
            "AppceleratorTitanium.gitignore",
            "Concrete5.gitignore",
            "CodeIgniter.gitignore",
            "Fortran.gitignore",
            "Julia.gitignore",
            "ForceDotCom.gitignore",
            "IAR.gitignore",
            "OracleForms.gitignore",
            "Smalltalk.gitignore",
            "Rails.gitignore",
            "Phalcon.gitignore",
            "Prestashop.gitignore",
            "Zig.gitignore",
            "Agda.gitignore",
            "Unity.gitignore",
            "FuelPHP.gitignore",
            "LemonStand.gitignore",
            "SeamGen.gitignore",
            "SCons.gitignore",
            "SymphonyCMS.gitignore",
            "FlaxEngine.gitignore",
            "CommonLisp.gitignore",
            ".git",
            "Gradle.gitignore",
            "Raku.gitignore",
            "Maven.gitignore",
            "Ruby.gitignore",
            "community",
            "ReScript.gitignore",
            "OpenCart.gitignore",
            "Fancy.gitignore",
            "TeX.gitignore",
            "Zephir.gitignore",
            "OCaml.gitignore",
            "Global",
            "UnrealEngine.gitignore",
            "Autotools.gitignore",
            "C.gitignore",
            "Kotlin.gitignore",
            "Qt.gitignore",
            "ROS.gitignore",
            "Idris.gitignore",
            "Opa.gitignore"];
        str_vec.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn extract_name_has_gitignore_in_name_removes_postfix(){
        let result = extract_name("C++.gitignore");
        assert_eq!(result.unwrap(), "C++");
    }

    #[test]
    fn test_help_function_works() {
        let result = get_string_vec();
        assert!(result.len() >= 142);
    }


    #[test]
    fn test_find_similar_using_c(){
        let result = find_similar("c", &get_string_vec());
        assert!(!result.is_empty());
        // assert!(result.contains("C++.gitignore".to_string().borrow()));    
        assert!(result.contains("C.gitignore".to_string().borrow()));    
        assert!(result.len() <= 1);
        assert!(result.len() < 2);
    }


    #[test]
    fn test_find_similar_using_rust(){
        let result = find_similar("rust", &get_string_vec());
        assert!(!result.is_empty());
        // assert!(result.contains("C++.gitignore".to_string().borrow()));    
        assert!(result.contains("Rust.gitignore".to_string().borrow()));    
        assert!(result.len() <= 1);
        assert!(result.len() < 2);
    }

    #[test]
    fn test_find_similar_using_python(){
        let result = find_similar("python", &get_string_vec());
        assert!(!result.is_empty());
        // assert!(result.contains("C++.gitignore".to_string().borrow()));    
        assert!(result.contains("Python.gitignore".to_string().borrow()));    
        assert!(result.len() <= 1);
        assert!(result.len() < 2);
    }


}