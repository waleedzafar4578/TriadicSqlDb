import React, { useEffect, useState } from 'react';
import MonacoEditor from 'react-monaco-editor';
import * as monaco from 'monaco-editor';

const MyEditor = ({ value, onChange }) => {
    useEffect(() => {
        const customLanguage = {
            tokenizer: {
                root: [

                    [/\bSELECT\b/, 'keyword'],
                    [/\bRENAME\b/, 'keyword'],
                    [/\bFROM\b/, 'keyword'],
                    [/\bWHERE\b/, 'keyword'],
                    [/\bJOIN\b/, 'keyword'],
                    [/\bLEFT\b/, 'keyword'],
                    [/\bRIGHT\b/, 'keyword'],
                    [/\bINNER\b/, 'keyword'],
                    [/\bOUTER\b/, 'keyword'],
                    [/\bON\b/, 'keyword'],
                    [/\bGROUP BY\b/, 'keyword'],
                    [/\bORDER BY\b/, 'keyword'],
                    [/\bASC\b/, 'keyword'],
                    [/\bDESC\b/, 'keyword'],
                    [/\bLIMIT\b/, 'keyword'],
                    [/\bOFFSET\b/, 'keyword'],
                    [/\bAND\b/, 'keyword'],
                    [/\bOR\b/, 'keyword'],
                    [/\bNOT\b/, 'keyword'],
                    [/\bAS\b/, 'keyword'],
                    [/\bIN\b/, 'keyword'],
                    [/\bLIKE\b/, 'keyword'],
                    [/\bIS\b/, 'keyword'],
                    [/\bNULL\b/, 'keyword'],
                    [/\bTRUE\b/, 'keyword'],
                    [/\bFALSE\b/, 'keyword'],
                    [/\bINT\b/, 'keyword'],
                    [/\bVARCHAR\b/, 'keyword'],
                    [/\bCREATE\b/, 'keyword'],
                    [/\bTABLE\b/, 'keyword'],
                    [/\bIF\b/, 'keyword'],
                    [/\bEXISTS\b/, 'keyword'],
                    [/\bPRIMARY KEY\b/, 'keyword'],
                    [/\bFOREIGN KEY\b/, 'keyword'],
                    [/\bREFERENCES\b/, 'keyword'],
                    [/\bINSERT INTO\b/, 'keyword'],
                    [/\bVALUES\b/, 'keyword'],
                    [/\bUPDATE\b/, 'keyword'],
                    [/\bSET\b/, 'keyword'],
                    [/\bDELETE\b/, 'keyword'],
                    [/\bDROP\b/, 'keyword'],
                    [/\bDATABASE\b/, 'keyword'],
                    [/\bUSE\b/, 'keyword'],
                    [/\bBEGIN\b/, 'keyword'],
                    [/\bCOMMIT\b/, 'keyword'],
                    [/\bROLLBACK\b/, 'keyword'],
                    [/\bTRANSACTION\b/, 'keyword'],
                    [/\bROLLUP\b/, 'keyword'],
                    [/\bCUBE\b/, 'keyword'],
                    [/\bGROUPING SETS\b/, 'keyword'],
                    [/\bUNION\b/, 'keyword'],
                    [/\bEXCEPT\b/, 'keyword'],
                    [/\bINTERSECT\b/, 'keyword'],
                    [/\bALL\b/, 'keyword'],
                    [/\bCASE\b/, 'keyword'],
                    [/\bWHEN\b/, 'keyword'],
                    [/\bTHEN\b/, 'keyword'],
                    [/\bELSE\b/, 'keyword'],
                    [/\bEND\b/, 'keyword'],
                    [/\bROW_NUMBER\b/, 'keyword'],
                    [/\bOVER\b/, 'keyword'],
                    [/\bPARTITION BY\b/, 'keyword'],
                    [/\bCURRENT_DATE\b/, 'keyword'],
                    [/\bCURRENT_TIME\b/, 'keyword'],
                    [/\bCURRENT_TIMESTAMP\b/, 'keyword'],
                    [/\bCAST\b/, 'keyword'],
                    [/\bAS\b/, 'keyword'],
                    [/\bJOIN\b/, 'keyword'],
                    [/\bLEFT\b/, 'keyword'],
                    [/\bRIGHT\b/, 'keyword'],
                    [/\bFULL\b/, 'keyword'],
                    [/\bOUTER\b/, 'keyword'],
                    [/\bINNER\b/, 'keyword'],
                    [/\bCROSS\b/, 'keyword'],
                    [/\bUSING\b/, 'keyword'],
                    [/\bNATURAL\b/, 'keyword'],
                    [/\bON\b/, 'keyword'],
                    [/\bDISTINCT\b/, 'keyword'],
                    [/\bANY\b/, 'keyword'],
                    [/\bSOME\b/, 'keyword'],
                    [/\bALL\b/, 'keyword'],
                    [/\bNULLS FIRST\b/, 'keyword'],
                    [/\bNULLS LAST\b/, 'keyword'],
                    [/\bNOT NULL\b/, 'keyword'],
                    [/\bHAVING\b/, 'keyword'],
                    [/\bAS\b/, 'keyword'],
                    [/\bLEFT JOIN\b/, 'keyword'],
                    [/\bRIGHT JOIN\b/, 'keyword'],
                    [/\bFULL JOIN\b/, 'keyword'],
                    [/\bOUTER JOIN\b/, 'keyword'],
                    [/\bINNER JOIN\b/, 'keyword'],
                    [/\bCROSS JOIN\b/, 'keyword'],
                    [/\bUNION ALL\b/, 'keyword'],
                    [/\bINTERSECT ALL\b/, 'keyword'],
                    [/\bEXCEPT ALL\b/, 'keyword'],
                    [/\bSHOW\b/, 'keyword'],
                    [/\bT\b/, 'keyword'],
                    [/\bL\b/, 'keyword'],
                    [/\bF\b/, 'keyword'],
                    [/[;,]/, 'delimiter'],
                    [/[+\-*/%=<>!?&|]/, 'operator'],
                    [/[()\\[\]{}]/, 'delimiter.parenthesis'],
                    [/\b\w+\b/, 'identifier'], // Tokenize identifiers (words)

                    // Add your custom tokenization rules here
                    // For example: [/\<[^\>]*\>/, 'tag'],
                ],
            },
        };

        // Register the custom language with Monaco Editor
        monaco.languages.register({ id: 'customLanguage' });
        monaco.languages.setMonarchTokensProvider('customLanguage', customLanguage);
    }, []);
    // Function to handle editor content change
    const handleEditorChange = (newValue) => {
        onChange(newValue); // Pass the new value to the parent component
    };

    const editorOptions = {
        wordWrap: 'wordWrapColumn',
        wordWrapColumn: 200, // Set a large value to force wrapping
        wordWrapMinified: false,
        layoutInfo: {
            wordWrapColumn: 0, // Center-align wrapped lines
        },
    };
    return (
        <MonacoEditor
            width="1000"
            height="400"
            language="customLanguage"
            theme="vs-dark"
            value={value} // Set the editor content from props
            options={editorOptions} // Pass the editor options
            onChange={handleEditorChange} // Handle editor content change
        />
    );
};

export default MyEditor;
