fn main() {
	let dquote = 34 as char;
	let backslash = 92 as char;
	let data = [
	"	",
	"1 C",
	"#include <stdio.h>",
	"#include <string.h>",
	"const char TAB = 9;",
	"const char LF = 10;",
	"const char DQUOTE = 34;",
	"const char PERCENT = 37;",
	"const char COMMA = 44;",
	"const char BACKSLASH = 92;",
	"const char s = 115;",
	"const char c = 99;",
	"const char format_chr[] = {PERCENT, c, 0};",
	"int main()",
	"{",
	"	unsigned i;",
	"	const char *data[] = {",
	"	};",
	"	for (i = 39; i <= 44; ++i)",
	"		puts(data[i]);",
	"	for (i = 0; i < sizeof(data) / sizeof(char*); ++i) {",
	"		const char* cur_str = data[i];",
	"		printf(format_chr, TAB);",
	"		printf(format_chr, DQUOTE);",
	"		unsigned j;",
	"		for (j = 0; j < strlen(cur_str); j++) {",
	"			if (cur_str[j] == DQUOTE) ",
	"				printf(format_chr, BACKSLASH);",
	"			printf(format_chr, cur_str[j]);",
	"		}",
	"		printf(format_chr, DQUOTE);",
	"		printf(format_chr, COMMA);",
	"		printf(format_chr, LF);",
	"	}",
	"	for (i = 45; i <= 61; ++i)",
	"		puts(data[i]);",
	"	return 0;",
	"}",
	"38 Kotlin",
	"fun main() {",
	"	val q = 34;",
	"	val DQUOTE = q.toChar();",
	"	val bs = 92;",
	"	val BACKSLASH = bs.toChar();",
	"	val data = arrayOf<String> (",
	"	);",
	"	for (i in 63..66)",
	"		println(data[i]);",
	"	for (i in 0..data.size - 1) {",
	"		print(data[0] + DQUOTE);",
	"		val cur_str = data[i];",
	"		for (j in 0..cur_str.length - 1) {",
	"			if (cur_str[j] == DQUOTE) ",
	"				print(BACKSLASH);",
	"			print(cur_str[j]);",
	"		}",
	"		print(DQUOTE);",
	"		println(',');",
	"	}",
	"	for (i in 67..data.size - 1)",
	"		println(data[i]);",
	"}",
	"62 Rust",
	"fn main() {",
	"	let dquote = 34 as char;",
	"	let backslash = 92 as char;",
	"	let data = [",
	"	];",
	"	for i in 2..17 {",
	"		println!(\"{}\", data[i].to_string());",
	"	}",
	"	for i in 0..data.len() {",
	"		let mut prefix = data[0].to_string();",
	"		prefix.push(dquote);",
	"		print!(\"{}\", prefix);",
	"		let line = data[i].to_string();",
	"		let cur_chr_arr = line.as_bytes();",
	"		for j in 0..cur_chr_arr.len() {",
	"			if cur_chr_arr[j] as char == dquote {",
	"				print!(\"{}\", backslash);",
	"			}",
	"			print!(\"{}\", cur_chr_arr[j] as char);",
	"		}",
	"		let mut postfix = String::new();",
	"		postfix.push(dquote);",
	"		postfix.push(',');",
	"		println!(\"{}\", postfix);",
	"	}",
	"	for i in 17..38 {",
	"		println!(\"{}\", data[i].to_string());",
	"	}",
	"}",
	];
	for i in 2..17 {
		println!("{}", data[i].to_string());
	}
	for i in 0..data.len() {
		let mut prefix = data[0].to_string();
		prefix.push(dquote);
		print!("{}", prefix);
		let line = data[i].to_string();
		let cur_chr_arr = line.as_bytes();
		for j in 0..cur_chr_arr.len() {
			if cur_chr_arr[j] as char == dquote {
				print!("{}", backslash);
			}
			print!("{}", cur_chr_arr[j] as char);
		}
		let mut postfix = String::new();
		postfix.push(dquote);
		postfix.push(',');
		println!("{}", postfix);
	}
	for i in 17..38 {
		println!("{}", data[i].to_string());
	}
}
