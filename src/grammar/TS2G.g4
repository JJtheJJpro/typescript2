grammar TS2G;

s: statement+ EOF;

statement:
	expr ';'						# exprstatement
	| LET ID ':' TYPE '=' expr ';'	# let
	| 'print' LPAR expr RPAR ';'	# print;

expr:
	INT								# number
	| ID							# id
	| PI							# pi
	| E								# e
	| LPAR expr RPAR				# parenthesis
	| <assoc = right> expr EXP expr	# exponent
	| expr (MUL | DIV) expr			# multiply
	| expr (ADD | SUB) expr			# add
	| ID EQ expr					# eq;

U8: 'u8';
I8: 'i8';
U16: 'u16';
I16: 'i16';
U32: 'u32';
I32: 'i32';
U64: 'u64';
I64: 'i64';
F32: 'f32';
F64: 'f64';
TYPE: U8 | I8 | U16 | I16 | U32 | I32 | U64 | I64 | F32 | F64;
INT: SUB? [0-9]+ (DOT [0-9]+)?;
PI: 'pi' | 'PI';
E: 'e';
LET: 'let';
ID: ([a-z] | [A-Z])+;
EQ: '=';
LPAR: '(';
RPAR: ')';
EXP: '^';
MUL: '*';
DIV: '/';
ADD: '+';
SUB: '-';
DOT: '.';
WS: [ \t]+ -> channel(HIDDEN);