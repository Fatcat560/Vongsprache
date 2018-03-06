//Vongsprache interpreter
//Copyright (C) 2018  Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>

///Permission is hereby granted, free of charge, to any person obtaining
//a copy of this software and associated documentation files (the "Software"),
//to deal in the Software without restriction, including without limitation the
//rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
//sell copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
//FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
//COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
//IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
//CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#include "lexer.h"

/**
 * Top-level parse function to parse the whole script
 * @param fp FILE* from which to read
 * @return program token containing the script
 */
Token* parseTopLevel(FILE* fp);

/**
 * Parse a function from the token stream
 * @param fp FILE* from which to read
 * @return function token containing the defined function
 */
Token* parseFunction(FILE* fp);

/**
 * Parse an if-block from the token stream
 * @param fp FILE* from which to read
 * @return conditional token defining the block
 */
Token* parseIf(FILE* fp);

/**
 * Parse a function call from the token stream
 * @param fp FILE* from which to read
 * @return call token indicating the called function and passed arguments
 */
Token* parseCall(FILE* fp);

/**
 * Parse depending on the current token
 * @param fp FILE* from which to read
 * @return token returned by the called delegate function
 */
Token* parseAtom(FILE* fp);

/**
 * Parse a sequence of statements
 * @param fp FILE* from which to read
 * @return program token containing statements
 */
Token* parseProg(FILE* fp);

/**
 * Parse the token stream for a variable name
 * @param fp FILE* from which to read
 * @return variable token
 */
Token* parseVariableName(FILE* fp);

/**
 * Parse the next expression contained in the token stream
 * @param fp FILE* from which to read
 * @return token representing the next expression in the stream
 */
Token* parseExpression(FILE* fp);

/**
 * Parse the stream for a function call or identifier
 * @param fp FILE* from which to read
 * @return the corresponding token
 */
Token* potentialCall(FILE* fp);

/**
 * Parse the stream for a potential binary expression
 * @param fp FILE* from which to read
 * @param token given token that might start an expression
 * @param prec precedence of the given token
 * @return the given token or another token representing the expression
 */
Token* potentialBinary(FILE* fp, Token* token, int prec);

/**
 * Parse the stream for a list of tokens between a specified start and end
 * characters delimited by the specified separator
 * @param fp FILE* from which to read
 * @param start starting character for the list
 * @param end ending character for the list
 * @param sep delimiter character for the list
 * @param parse parse function to parse the delimited tokens
 * @return an array containing the parsed tokens
 */
Token** parseDelimited(FILE* fp, char start, char end, char sep,
	Token* (*parse)(FILE*));