#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A compiled representation of a regular expression.
/// <p>A regular expression, specified as a string, must first be compiled into an instance of this class. The resulting pattern can then be used to create a <a title="class in java.util.regex" href="../../../java/util/regex/Matcher.html"><code>Matcher</code></a> object that can match arbitrary <a href="../../../java/lang/CharSequence.html" title="interface in java.lang">character sequences</a> against the regular expression. All of the state involved in performing a match resides in the matcher, so many matchers can share the same pattern.</p>
/// <p>A typical invocation sequence is thus</p>
/// <blockquote>
/// <pre> Pattern p = Pattern.<a href="../../../java/util/regex/Pattern.html#compile-java.lang.String-"><code>compile</code></a>("a*b");
/// Matcher m = p.<a href="../../../java/util/regex/Pattern.html#matcher-java.lang.CharSequence-"><code>matcher</code></a>("aaaaab");
/// boolean b = m.<a href="../../../java/util/regex/Matcher.html#matches--"><code>matches</code></a>();</pre>
/// </blockquote>
/// <p>A <a href="../../../java/util/regex/Pattern.html#matches-java.lang.String-java.lang.CharSequence-"><code>matches</code></a> method is defined by this class as a convenience for when a regular expression is used just once. This method compiles an expression and matches an input sequence against it in a single invocation. The statement</p>
/// <blockquote>
/// <pre> boolean b = Pattern.matches("a*b", "aaaaab");</pre>
/// </blockquote> is equivalent to the three statements above, though for repeated matches it is less efficient since it does not allow the compiled pattern to be reused.
/// <p>Instances of this class are immutable and are safe for use by multiple concurrent threads. Instances of the <a href="../../../java/util/regex/Matcher.html" title="class in java.util.regex"><code>Matcher</code></a> class are not safe for such use.</p>
/// <h3><a name="sum">Summary of regular-expression constructs</a></h3>
/// <table cellpadding="1" summary="Regular expression constructs, and what they match" cellspacing="0" border="0">
/// <tbody>
/// <tr align="left">
/// <th align="left" id="construct">Construct</th>
/// <th id="matches" align="left">Matches</th>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="characters" colspan="2">Characters</th>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><i>x</i></td>
/// <td headers="matches">The character <i>x</i></td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\\</tt></td>
/// <td headers="matches">The backslash character</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\0</tt><i>n</i></td>
/// <td headers="matches">The character with octal value <tt>0</tt><i>n</i> (0&nbsp;<tt>&lt;=</tt>&nbsp;<i>n</i>&nbsp;<tt>&lt;=</tt>&nbsp;7)</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\0</tt><i>nn</i></td>
/// <td headers="matches">The character with octal value <tt>0</tt><i>nn</i> (0&nbsp;<tt>&lt;=</tt>&nbsp;<i>n</i>&nbsp;<tt>&lt;=</tt>&nbsp;7)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct characters"><tt>\0</tt><i>mnn</i></td>
/// <td headers="matches">The character with octal value <tt>0</tt><i>mnn</i> (0&nbsp;<tt>&lt;=</tt>&nbsp;<i>m</i>&nbsp;<tt>&lt;=</tt>&nbsp;3, 0&nbsp;<tt>&lt;=</tt>&nbsp;<i>n</i>&nbsp;<tt>&lt;=</tt>&nbsp;7)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct characters"><tt>\x</tt><i>hh</i></td>
/// <td headers="matches">The character with hexadecimal&nbsp;value&nbsp;<tt>0x</tt><i>hh</i></td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\u</tt><i>hhhh</i></td>
/// <td headers="matches">The character with hexadecimal&nbsp;value&nbsp;<tt>0x</tt><i>hhhh</i></td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\x</tt><i>{h...h}</i></td>
/// <td headers="matches">The character with hexadecimal&nbsp;value&nbsp;<tt>0x</tt><i>h...h</i> (<a href="../../../java/lang/Character.html#MIN_CODE_POINT"><code>Character.MIN_CODE_POINT</code></a> &nbsp;&lt;=&nbsp;<tt>0x</tt><i>h...h</i>&nbsp;&lt;=&nbsp; <a href="../../../java/lang/Character.html#MAX_CODE_POINT"><code>Character.MAX_CODE_POINT</code></a>)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="matches"><tt>\t</tt></td>
/// <td headers="matches">The tab character (<tt>'\u0009'</tt>)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct characters"><tt>\n</tt></td>
/// <td headers="matches">The newline (line feed) character (<tt>'\u000A'</tt>)</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\r</tt></td>
/// <td headers="matches">The carriage-return character (<tt>'\u000D'</tt>)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct characters"><tt>\f</tt></td>
/// <td headers="matches">The form-feed character (<tt>'\u000C'</tt>)</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\a</tt></td>
/// <td headers="matches">The alert (bell) character (<tt>'\u0007'</tt>)</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\e</tt></td>
/// <td headers="matches">The escape character (<tt>'\u001B'</tt>)</td>
/// </tr>
/// <tr>
/// <td headers="construct characters" valign="top"><tt>\c</tt><i>x</i></td>
/// <td headers="matches">The control character corresponding to <i>x</i></td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="classes">Character classes</th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct classes"><code>[abc]</code></td>
/// <td headers="matches"><code>a</code>, <code>b</code>, or <code>c</code> (simple class)</td>
/// </tr>
/// <tr>
/// <td headers="construct classes" valign="top"><code>[^abc]</code></td>
/// <td headers="matches">Any character except <code>a</code>, <code>b</code>, or <code>c</code> (negation)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct classes"><code>[a-zA-Z]</code></td>
/// <td headers="matches"><code>a</code> through <code>z</code> or <code>A</code> through <code>Z</code>, inclusive (range)</td>
/// </tr>
/// <tr>
/// <td headers="construct classes" valign="top"><code>[a-d[m-p]]</code></td>
/// <td headers="matches"><code>a</code> through <code>d</code>, or <code>m</code> through <code>p</code>: <code>[a-dm-p]</code> (union)</td>
/// </tr>
/// <tr>
/// <td headers="construct classes" valign="top"><code>[a-z&amp;&amp;[def]]</code></td>
/// <td headers="matches"><code>d</code>, <code>e</code>, or <code>f</code> (intersection)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct classes"><code>[a-z&amp;&amp;[^bc]]</code></td>
/// <td headers="matches"><code>a</code> through <code>z</code>, except for <code>b</code> and <code>c</code>: <code>[ad-z]</code> (subtraction)</td>
/// </tr>
/// <tr>
/// <td headers="construct classes" valign="top"><code>[a-z&amp;&amp;[^m-p]]</code></td>
/// <td headers="matches"><code>a</code> through <code>z</code>, and not <code>m</code> through <code>p</code>: <code>[a-lq-z]</code>(subtraction)</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="predef" colspan="2">Predefined character classes</th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>.</tt></td>
/// <td headers="matches">Any character (may or may not match <a href="#lt">line terminators</a>)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\d</tt></td>
/// <td headers="matches">A digit: <tt>[0-9]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\D</tt></td>
/// <td headers="matches">A non-digit: <tt>[^0-9]</tt></td>
/// </tr>
/// <tr>
/// <td headers="construct predef" valign="top"><tt>\h</tt></td>
/// <td headers="matches">A horizontal whitespace character: <tt>[ \t\xA0\u1680\u180e\u2000-\u200a\u202f\u205f\u3000]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\H</tt></td>
/// <td headers="matches">A non-horizontal whitespace character: <tt>[^\h]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\s</tt></td>
/// <td headers="matches">A whitespace character: <tt>[ \t\n\x0B\f\r]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\S</tt></td>
/// <td headers="matches">A non-whitespace character: <tt>[^\s]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\v</tt></td>
/// <td headers="matches">A vertical whitespace character: <tt>[\n\x0B\f\r\x85\u2028\u2029]</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct predef"><tt>\V</tt></td>
/// <td headers="matches">A non-vertical whitespace character: <tt>[^\v]</tt></td>
/// </tr>
/// <tr>
/// <td headers="construct predef" valign="top"><tt>\w</tt></td>
/// <td headers="matches">A word character: <tt>[a-zA-Z_0-9]</tt></td>
/// </tr>
/// <tr>
/// <td headers="construct predef" valign="top"><tt>\W</tt></td>
/// <td headers="matches">A non-word character: <tt>[^\w]</tt></td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="posix" colspan="2"><b>POSIX character classes (US-ASCII only)</b></th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct posix"><code>\p{Lower}</code></td>
/// <td headers="matches">A lower-case alphabetic character: <code>[a-z]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Upper}</code></td>
/// <td headers="matches">An upper-case alphabetic character:<code>[A-Z]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{ASCII}</code></td>
/// <td headers="matches">All ASCII:<code>[\x00-\x7F]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Alpha}</code></td>
/// <td headers="matches">An alphabetic character:<code>[\p{Lower}\p{Upper}]</code></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct posix"><code>\p{Digit}</code></td>
/// <td headers="matches">A decimal digit: <code>[0-9]</code></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct posix"><code>\p{Alnum}</code></td>
/// <td headers="matches">An alphanumeric character:<code>[\p{Alpha}\p{Digit}]</code></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct posix"><code>\p{Punct}</code></td>
/// <td headers="matches">Punctuation: One of <code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</code></td>
/// </tr><!-- <code>[\!"#\$%&amp;'\(\)\*\+,\-\./:;\&lt;=\&gt;\?@\[\\\]\^_`\{\|\}~]</code>
/// <code>[\X21-\X2F\X31-\X40\X5B-\X60\X7B-\X7E]</code> -->
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Graph}</code></td>
/// <td headers="matches">A visible character: <code>[\p{Alnum}\p{Punct}]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Print}</code></td>
/// <td headers="matches">A printable character: <code>[\p{Graph}\x20]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Blank}</code></td>
/// <td headers="matches">A space or a tab: <code>[ \t]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Cntrl}</code></td>
/// <td headers="matches">A control character: <code>[\x00-\x1F\x7F]</code></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct posix"><code>\p{XDigit}</code></td>
/// <td headers="matches">A hexadecimal digit: <code>[0-9a-fA-F]</code></td>
/// </tr>
/// <tr>
/// <td headers="construct posix" valign="top"><code>\p{Space}</code></td>
/// <td headers="matches">A whitespace character: <code>[ \t\n\x0B\f\r]</code></td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2">java.lang.Character classes (simple <a href="#jcc">java character type</a>)</th>
/// </tr>
/// <tr>
/// <td valign="top"><tt>\p{javaLowerCase}</tt></td>
/// <td>Equivalent to java.lang.Character.isLowerCase()</td>
/// </tr>
/// <tr>
/// <td valign="top"><tt>\p{javaUpperCase}</tt></td>
/// <td>Equivalent to java.lang.Character.isUpperCase()</td>
/// </tr>
/// <tr>
/// <td valign="top"><tt>\p{javaWhitespace}</tt></td>
/// <td>Equivalent to java.lang.Character.isWhitespace()</td>
/// </tr>
/// <tr>
/// <td valign="top"><tt>\p{javaMirrored}</tt></td>
/// <td>Equivalent to java.lang.Character.isMirrored()</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="unicode" colspan="2">Classes for Unicode scripts, blocks, categories and binary properties</th>
/// </tr>
/// <tr>
/// <td headers="construct unicode" valign="top"><code>\p{IsLatin}</code></td>
/// <td headers="matches">A Latin&nbsp;script character (<a href="#usc">script</a>)</td>
/// </tr>
/// <tr>
/// <td headers="construct unicode" valign="top"><code>\p{InGreek}</code></td>
/// <td headers="matches">A character in the Greek&nbsp;block (<a href="#ubc">block</a>)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct unicode"><code>\p{Lu}</code></td>
/// <td headers="matches">An uppercase letter (<a href="#ucc">category</a>)</td>
/// </tr>
/// <tr>
/// <td headers="construct unicode" valign="top"><code>\p{IsAlphabetic}</code></td>
/// <td headers="matches">An alphabetic character (<a href="#ubpc">binary property</a>)</td>
/// </tr>
/// <tr>
/// <td headers="construct unicode" valign="top"><code>\p{Sc}</code></td>
/// <td headers="matches">A currency symbol</td>
/// </tr>
/// <tr>
/// <td headers="construct unicode" valign="top"><code>\P{InGreek}</code></td>
/// <td headers="matches">Any character except one in the Greek block (negation)</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct unicode"><code>[\p{L}&amp;&amp;[^\p{Lu}]]</code></td>
/// <td headers="matches">Any letter except an uppercase letter (subtraction)</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="bounds">Boundary matchers</th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>^</tt></td>
/// <td headers="matches">The beginning of a line</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>$</tt></td>
/// <td headers="matches">The end of a line</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>\b</tt></td>
/// <td headers="matches">A word boundary</td>
/// </tr>
/// <tr>
/// <td headers="construct bounds" valign="top"><tt>\B</tt></td>
/// <td headers="matches">A non-word boundary</td>
/// </tr>
/// <tr>
/// <td headers="construct bounds" valign="top"><tt>\A</tt></td>
/// <td headers="matches">The beginning of the input</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>\G</tt></td>
/// <td headers="matches">The end of the previous match</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>\Z</tt></td>
/// <td headers="matches">The end of the input but for the final <a href="#lt">terminator</a>, if&nbsp;any</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct bounds"><tt>\z</tt></td>
/// <td headers="matches">The end of the input</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="lineending" colspan="2">Linebreak matcher</th>
/// </tr>
/// <tr>
/// <td headers="construct lineending" valign="top"><tt>\R</tt></td>
/// <td headers="matches">Any Unicode linebreak sequence, is equivalent to <tt>\u000D\u000A|[\u000A\u000B\u000C\u000D\u0085\u2028\u2029] </tt></td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="greedy" colspan="2">Greedy quantifiers</th>
/// </tr>
/// <tr>
/// <td headers="construct greedy" valign="top"><i>X</i><tt>?</tt></td>
/// <td headers="matches"><i>X</i>, once or not at all</td>
/// </tr>
/// <tr>
/// <td headers="construct greedy" valign="top"><i>X</i><tt>*</tt></td>
/// <td headers="matches"><i>X</i>, zero or more times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct greedy"><i>X</i><tt>+</tt></td>
/// <td headers="matches"><i>X</i>, one or more times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct greedy"><i>X</i><tt>{</tt><i>n</i><tt>}</tt></td>
/// <td headers="matches"><i>X</i>, exactly <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td headers="construct greedy" valign="top"><i>X</i><tt>{</tt><i>n</i><tt>,}</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td headers="construct greedy" valign="top"><i>X</i><tt>{</tt><i>n</i><tt>,</tt><i>m</i><tt>}</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> but not more than <i>m</i> times</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="reluc">Reluctant quantifiers</th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct reluc"><i>X</i><tt>??</tt></td>
/// <td headers="matches"><i>X</i>, once or not at all</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct reluc"><i>X</i><tt>*?</tt></td>
/// <td headers="matches"><i>X</i>, zero or more times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct reluc"><i>X</i><tt>+?</tt></td>
/// <td headers="matches"><i>X</i>, one or more times</td>
/// </tr>
/// <tr>
/// <td headers="construct reluc" valign="top"><i>X</i><tt>{</tt><i>n</i><tt>}?</tt></td>
/// <td headers="matches"><i>X</i>, exactly <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct reluc"><i>X</i><tt>{</tt><i>n</i><tt>,}?</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct reluc"><i>X</i><tt>{</tt><i>n</i><tt>,</tt><i>m</i><tt>}?</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> but not more than <i>m</i> times</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="poss">Possessive quantifiers</th>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct poss"><i>X</i><tt>?+</tt></td>
/// <td headers="matches"><i>X</i>, once or not at all</td>
/// </tr>
/// <tr>
/// <td headers="construct poss" valign="top"><i>X</i><tt>*+</tt></td>
/// <td headers="matches"><i>X</i>, zero or more times</td>
/// </tr>
/// <tr>
/// <td headers="construct poss" valign="top"><i>X</i><tt>++</tt></td>
/// <td headers="matches"><i>X</i>, one or more times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct poss"><i>X</i><tt>{</tt><i>n</i><tt>}+</tt></td>
/// <td headers="matches"><i>X</i>, exactly <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct poss"><i>X</i><tt>{</tt><i>n</i><tt>,}+</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> times</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct poss"><i>X</i><tt>{</tt><i>n</i><tt>,</tt><i>m</i><tt>}+</tt></td>
/// <td headers="matches"><i>X</i>, at least <i>n</i> but not more than <i>m</i> times</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="logical" colspan="2">Logical operators</th>
/// </tr>
/// <tr>
/// <td headers="construct logical" valign="top"><i>XY</i></td>
/// <td headers="matches"><i>X</i> followed by <i>Y</i></td>
/// </tr>
/// <tr>
/// <td headers="construct logical" valign="top"><i>X</i><tt>|</tt><i>Y</i></td>
/// <td headers="matches">Either <i>X</i> or <i>Y</i></td>
/// </tr>
/// <tr>
/// <td headers="construct logical" valign="top"><tt>(</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches">X, as a <a href="#cg">capturing group</a></td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="backref">Back references</th>
/// </tr>
/// <tr>
/// <td valign="bottom" headers="construct backref"><tt>\</tt><i>n</i></td>
/// <td valign="bottom" headers="matches">Whatever the <i>n</i><sup>th</sup> <a href="#cg">capturing group</a> matched</td>
/// </tr>
/// <tr>
/// <td headers="construct backref" valign="bottom"><tt>\</tt><i>k</i>&lt;<i>name</i>&gt;</td>
/// <td valign="bottom" headers="matches">Whatever the <a href="#groupname">named-capturing group</a> "name" matched</td>
/// </tr>
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th id="quot" colspan="2">Quotation</th>
/// </tr>
/// <tr>
/// <td headers="construct quot" valign="top"><tt>\</tt></td>
/// <td headers="matches">Nothing, but quotes the following character</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct quot"><tt>\Q</tt></td>
/// <td headers="matches">Nothing, but quotes all characters until <tt>\E</tt></td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct quot"><tt>\E</tt></td>
/// <td headers="matches">Nothing, but ends quoting started by <tt>\Q</tt></td>
/// </tr><!-- Metachars: !$()*+.<>?[\]^{|} -->
/// <tr>
/// <th>&nbsp;</th>
/// </tr>
/// <tr align="left">
/// <th colspan="2" id="special">Special constructs (named-capturing and non-capturing)</th>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?&lt;<a href="#groupname">name</a>&gt;</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, as a named-capturing group</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?:</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, as a non-capturing group</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?idmsuxU-idmsuxU)&nbsp;</tt></td>
/// <td headers="matches">Nothing, but turns match flags <a href="#CASE_INSENSITIVE">i</a> <a href="#UNIX_LINES">d</a> <a href="#MULTILINE">m</a> <a href="#DOTALL">s</a> <a href="#UNICODE_CASE">u</a> <a href="#COMMENTS">x</a> <a href="#UNICODE_CHARACTER_CLASS">U</a> on - off</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct special"><tt>(?idmsux-idmsux:</tt><i>X</i><tt>)</tt>&nbsp;&nbsp;</td>
/// <td headers="matches"><i>X</i>, as a <a href="#cg">non-capturing group</a> with the given flags <a href="#CASE_INSENSITIVE">i</a> <a href="#UNIX_LINES">d</a> <a href="#MULTILINE">m</a> <a href="#DOTALL">s</a> <a href="#UNICODE_CASE">u</a> <a href="#COMMENTS">x</a> on - off</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?=</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, via zero-width positive lookahead</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?!</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, via zero-width negative lookahead</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?&lt;=</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, via zero-width positive lookbehind</td>
/// </tr>
/// <tr>
/// <td valign="top" headers="construct special"><tt>(?&lt;!</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, via zero-width negative lookbehind</td>
/// </tr>
/// <tr>
/// <td headers="construct special" valign="top"><tt>(?&gt;</tt><i>X</i><tt>)</tt></td>
/// <td headers="matches"><i>X</i>, as an independent, non-capturing group</td>
/// </tr>
/// </tbody>
/// </table>
/// <hr>
/// <h3><a name="bs">Backslashes, escapes, and quoting</a></h3>
/// <p>The backslash character (<tt>'\'</tt>) serves to introduce escaped constructs, as defined in the table above, as well as to quote characters that otherwise would be interpreted as unescaped constructs. Thus the expression <tt>\\</tt> matches a single backslash and <tt>\{</tt> matches a left brace.</p>
/// <p>It is an error to use a backslash prior to any alphabetic character that does not denote an escaped construct; these are reserved for future extensions to the regular-expression language. A backslash may be used prior to a non-alphabetic character regardless of whether that character is part of an unescaped construct.</p>
/// <p>Backslashes within string literals in Java source code are interpreted as required by <cite>The Java™ Language Specification</cite> as either Unicode escapes (section 3.3) or other character escapes (section 3.10.6) It is therefore necessary to double backslashes in string literals that represent regular expressions to protect them from interpretation by the Java bytecode compiler. The string literal <tt>"\b"</tt>, for example, matches a single backspace character when interpreted as a regular expression, while <tt>"\\b"</tt> matches a word boundary. The string literal <tt>"\(hello\)"</tt> is illegal and leads to a compile-time error; in order to match the string <tt>(hello)</tt> the string literal <tt>"\\(hello\\)"</tt> must be used.</p>
/// <h3><a name="cc">Character Classes</a></h3>
/// <p>Character classes may appear within other character classes, and may be composed by the union operator (implicit) and the intersection operator (<tt>&amp;&amp;</tt>). The union operator denotes a class that contains every character that is in at least one of its operand classes. The intersection operator denotes a class that contains every character that is in both of its operand classes.</p>
/// <p>The precedence of character-class operators is as follows, from highest to lowest:</p>
/// <blockquote>
/// <table summary="Precedence of character class operators." border="0" cellpadding="1" cellspacing="0">
/// <tbody>
/// <tr>
/// <th>1&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td>Literal escape&nbsp;&nbsp;&nbsp;&nbsp;</td>
/// <td><tt>\x</tt></td>
/// </tr>
/// <tr>
/// <th>2&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td>Grouping</td>
/// <td><tt>[...]</tt></td>
/// </tr>
/// <tr>
/// <th>3&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td>Range</td>
/// <td><tt>a-z</tt></td>
/// </tr>
/// <tr>
/// <th>4&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td>Union</td>
/// <td><tt>[a-e][i-u]</tt></td>
/// </tr>
/// <tr>
/// <th>5&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td>Intersection</td>
/// <td><code>[a-z&amp;&amp;[aeiou]]</code></td>
/// </tr>
/// </tbody>
/// </table>
/// </blockquote>
/// <p>Note that a different set of metacharacters are in effect inside a character class than outside a character class. For instance, the regular expression <tt>.</tt> loses its special meaning inside a character class, while the expression <tt>-</tt> becomes a range forming metacharacter.</p>
/// <h3><a name="lt">Line terminators</a></h3>
/// <p>A <i>line terminator</i> is a one- or two-character sequence that marks the end of a line of the input character sequence. The following are recognized as line terminators:</p>
/// <ul>
/// <li>A newline (line feed) character&nbsp;(<tt>'\n'</tt>),</li>
/// <li>A carriage-return character followed immediately by a newline character&nbsp;(<tt>"\r\n"</tt>),</li>
/// <li>A standalone carriage-return character&nbsp;(<tt>'\r'</tt>),</li>
/// <li>A next-line character&nbsp;(<tt>'\u0085'</tt>),</li>
/// <li>A line-separator character&nbsp;(<tt>'\u2028'</tt>), or</li>
/// <li>A paragraph-separator character&nbsp;(<tt>'\u2029</tt>).</li>
/// </ul>
/// <p>If <a href="../../../java/util/regex/Pattern.html#UNIX_LINES"><code>UNIX_LINES</code></a> mode is activated, then the only line terminators recognized are newline characters.</p>
/// <p>The regular expression <tt>.</tt> matches any character except a line terminator unless the <a href="../../../java/util/regex/Pattern.html#DOTALL"><code>DOTALL</code></a> flag is specified.</p>
/// <p>By default, the regular expressions <tt>^</tt> and <tt>$</tt> ignore line terminators and only match at the beginning and the end, respectively, of the entire input sequence. If <a href="../../../java/util/regex/Pattern.html#MULTILINE"><code>MULTILINE</code></a> mode is activated then <tt>^</tt> matches at the beginning of input and after any line terminator except at the end of input. When in <a href="../../../java/util/regex/Pattern.html#MULTILINE"><code>MULTILINE</code></a> mode <tt>$</tt> matches just before a line terminator or the end of the input sequence.</p>
/// <h3><a name="cg">Groups and capturing</a></h3>
/// <h4><a name="gnumber">Group number</a></h4>
/// <p>Capturing groups are numbered by counting their opening parentheses from left to right. In the expression <tt>((A)(B(C)))</tt>, for example, there are four such groups:</p>
/// <blockquote>
/// <table summary="Capturing group numberings" cellpadding="1" cellspacing="0">
/// <tbody>
/// <tr>
/// <th>1&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td><tt>((A)(B(C)))</tt></td>
/// </tr>
/// <tr>
/// <th>2&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td><tt>(A)</tt></td>
/// </tr>
/// <tr>
/// <th>3&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td><tt>(B(C))</tt></td>
/// </tr>
/// <tr>
/// <th>4&nbsp;&nbsp;&nbsp;&nbsp;</th>
/// <td><tt>(C)</tt></td>
/// </tr>
/// </tbody>
/// </table>
/// </blockquote>
/// <p>Group zero always stands for the entire expression.</p>
/// <p>Capturing groups are so named because, during a match, each subsequence of the input sequence that matches such a group is saved. The captured subsequence may be used later in the expression, via a back reference, and may also be retrieved from the matcher once the match operation is complete.</p>
/// <h4><a name="groupname">Group name</a></h4>
/// <p>A capturing group can also be assigned a "name", a <tt>named-capturing group</tt>, and then be back-referenced later by the "name". Group names are composed of the following characters. The first character must be a <tt>letter</tt>.</p>
/// <ul>
/// <li>The uppercase letters <tt>'A'</tt> through <tt>'Z'</tt> (<tt>'\u0041'</tt>&nbsp;through&nbsp;<tt>'\u005a'</tt>),</li>
/// <li>The lowercase letters <tt>'a'</tt> through <tt>'z'</tt> (<tt>'\u0061'</tt>&nbsp;through&nbsp;<tt>'\u007a'</tt>),</li>
/// <li>The digits <tt>'0'</tt> through <tt>'9'</tt> (<tt>'\u0030'</tt>&nbsp;through&nbsp;<tt>'\u0039'</tt>),</li>
/// </ul>
/// <p>A <tt>named-capturing group</tt> is still numbered as described in <a href="#gnumber">Group number</a>.</p>
/// <p>The captured input associated with a group is always the subsequence that the group most recently matched. If a group is evaluated a second time because of quantification then its previously-captured value, if any, will be retained if the second evaluation fails. Matching the string <tt>"aba"</tt> against the expression <tt>(a(b)?)+</tt>, for example, leaves group two set to <tt>"b"</tt>. All captured input is discarded at the beginning of each match.</p>
/// <p>Groups beginning with <tt>(?</tt> are either pure, <i>non-capturing</i> groups that do not capture text and do not count towards the group total, or <i>named-capturing</i> group.</p>
/// <h3>Unicode support</h3>
/// <p>This class is in conformance with Level 1 of <a href="http://www.unicode.org/reports/tr18/"><i>Unicode Technical Standard #18: Unicode Regular Expression</i></a>, plus RL2.1 Canonical Equivalents.</p>
/// <p><b>Unicode escape sequences</b> such as <tt>\u2014</tt> in Java source code are processed as described in section 3.3 of <cite>The Java™ Language Specification</cite>. Such escape sequences are also implemented directly by the regular-expression parser so that Unicode escapes can be used in expressions that are read from files or from the keyboard. Thus the strings <tt>"\u2014"</tt> and <tt>"\\u2014"</tt>, while not equal, compile into the same pattern, which matches the character with hexadecimal value <tt>0x2014</tt>.</p>
/// <p>A Unicode character can also be represented in a regular-expression by using its <b>Hex notation</b>(hexadecimal code point value) directly as described in construct <tt>\x{...}</tt>, for example a supplementary character U+2011F can be specified as <tt>\x{2011F}</tt>, instead of two consecutive Unicode escape sequences of the surrogate pair <tt>\uD840</tt><tt>\uDD1F</tt>.</p>
/// <p>Unicode scripts, blocks, categories and binary properties are written with the <tt>\p</tt> and <tt>\P</tt> constructs as in Perl. <tt>\p{</tt><i>prop</i><tt>}</tt> matches if the input has the property <i>prop</i>, while <tt>\P{</tt><i>prop</i><tt>}</tt> does not match if the input has that property.</p>
/// <p>Scripts, blocks, categories and binary properties can be used both inside and outside of a character class.</p>
/// <p><b><a name="usc">Scripts</a></b> are specified either with the prefix <code>Is</code>, as in <code>IsHiragana</code>, or by using the <code>script</code> keyword (or its short form <code>sc</code>)as in <code>script=Hiragana</code> or <code>sc=Hiragana</code>.</p>
/// <p>The script names supported by <code>Pattern</code> are the valid script names accepted and defined by <a href="../../../java/lang/Character.UnicodeScript.html#forName-java.lang.String-"><code>UnicodeScript.forName</code></a>.</p>
/// <p><b><a name="ubc">Blocks</a></b> are specified with the prefix <code>In</code>, as in <code>InMongolian</code>, or by using the keyword <code>block</code> (or its short form <code>blk</code>) as in <code>block=Mongolian</code> or <code>blk=Mongolian</code>.</p>
/// <p>The block names supported by <code>Pattern</code> are the valid block names accepted and defined by <a href="../../../java/lang/Character.UnicodeBlock.html#forName-java.lang.String-"><code>UnicodeBlock.forName</code></a>.</p>
/// <p><b><a name="ucc">Categories</a></b> may be specified with the optional prefix <code>Is</code>: Both <code>\p{L}</code> and <code>\p{IsL}</code> denote the category of Unicode letters. Same as scripts and blocks, categories can also be specified by using the keyword <code>general_category</code> (or its short form <code>gc</code>) as in <code>general_category=Lu</code> or <code>gc=Lu</code>.</p>
/// <p>The supported categories are those of <a href="http://www.unicode.org/unicode/standard/standard.html"> <i>The Unicode Standard</i></a> in the version specified by the <a href="../../../java/lang/Character.html" title="class in java.lang"><code>Character</code></a> class. The category names are those defined in the Standard, both normative and informative.</p>
/// <p><b><a name="ubpc">Binary properties</a></b> are specified with the prefix <code>Is</code>, as in <code>IsAlphabetic</code>. The supported binary properties by <code>Pattern</code> are</p>
/// <ul>
/// <li>Alphabetic</li>
/// <li>Ideographic</li>
/// <li>Letter</li>
/// <li>Lowercase</li>
/// <li>Uppercase</li>
/// <li>Titlecase</li>
/// <li>Punctuation</li>
/// <li>Control</li>
/// <li>White_Space</li>
/// <li>Digit</li>
/// <li>Hex_Digit</li>
/// <li>Join_Control</li>
/// <li>Noncharacter_Code_Point</li>
/// <li>Assigned</li>
/// </ul>
/// <p>The following <b>Predefined Character classes</b> and <b>POSIX character classes</b> are in conformance with the recommendation of <i>Annex C: Compatibility Properties</i> of <a href="http://www.unicode.org/reports/tr18/"><i>Unicode Regular Expression </i></a>, when <a href="../../../java/util/regex/Pattern.html#UNICODE_CHARACTER_CLASS"><code>UNICODE_CHARACTER_CLASS</code></a> flag is specified.</p>
/// <table cellspacing="0" border="0" cellpadding="1" summary="predefined and posix character classes in Unicode mode">
/// <tbody>
/// <tr align="left">
/// <th id="predef_classes" align="left">Classes</th>
/// <th align="left" id="predef_matches">Matches</th>
/// </tr>
/// <tr>
/// <td><tt>\p{Lower}</tt></td>
/// <td>A lowercase character:<tt>\p{IsLowercase}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Upper}</tt></td>
/// <td>An uppercase character:<tt>\p{IsUppercase}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{ASCII}</tt></td>
/// <td>All ASCII:<tt>[\x00-\x7F]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Alpha}</tt></td>
/// <td>An alphabetic character:<tt>\p{IsAlphabetic}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Digit}</tt></td>
/// <td>A decimal digit character:<tt>p{IsDigit}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Alnum}</tt></td>
/// <td>An alphanumeric character:<tt>[\p{IsAlphabetic}\p{IsDigit}]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Punct}</tt></td>
/// <td>A punctuation character:<tt>p{IsPunctuation}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Graph}</tt></td>
/// <td>A visible character: <tt>[^\p{IsWhite_Space}\p{gc=Cc}\p{gc=Cs}\p{gc=Cn}]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Print}</tt></td>
/// <td>A printable character: <code>[\p{Graph}\p{Blank}&amp;&amp;[^\p{Cntrl}]]</code></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Blank}</tt></td>
/// <td>A space or a tab: <code>[\p{IsWhite_Space}&amp;&amp;[^\p{gc=Zl}\p{gc=Zp}\x0a\x0b\x0c\x0d\x85]]</code></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Cntrl}</tt></td>
/// <td>A control character: <tt>\p{gc=Cc}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{XDigit}</tt></td>
/// <td>A hexadecimal digit: <tt>[\p{gc=Nd}\p{IsHex_Digit}]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\p{Space}</tt></td>
/// <td>A whitespace character:<tt>\p{IsWhite_Space}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\d</tt></td>
/// <td>A digit: <tt>\p{IsDigit}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\D</tt></td>
/// <td>A non-digit: <tt>[^\d]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\s</tt></td>
/// <td>A whitespace character: <tt>\p{IsWhite_Space}</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\S</tt></td>
/// <td>A non-whitespace character: <tt>[^\s]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\w</tt></td>
/// <td>A word character: <tt>[\p{Alpha}\p{gc=Mn}\p{gc=Me}\p{gc=Mc}\p{Digit}\p{gc=Pc}\p{IsJoin_Control}]</tt></td>
/// </tr>
/// <tr>
/// <td><tt>\W</tt></td>
/// <td>A non-word character: <tt>[^\w]</tt></td>
/// </tr>
/// </tbody>
/// </table>
/// <p><a name="jcc"> Categories that behave like the java.lang.Character boolean is<i>methodname</i> methods (except for the deprecated ones) are available through the same <tt>\p{</tt><i>prop</i><tt>}</tt> syntax where the specified property has the name <tt>java<i>methodname</i></tt></a>.</p>
/// <h3>Comparison to Perl 5</h3>
/// <p>The <code>Pattern</code> engine performs traditional NFA-based matching with ordered alternation as occurs in Perl 5.</p>
/// <p>Perl constructs not supported by this class:</p>
/// <ul>
/// <li>
/// <p>Predefined character classes (Unicode character)</p>
/// <p><tt>\X&nbsp;&nbsp;&nbsp;&nbsp;</tt>Match Unicode <a href="http://www.unicode.org/reports/tr18/#Default_Grapheme_Clusters"> <i>extended grapheme cluster</i></a></p></li>
/// <li>
/// <p>The backreference constructs, <tt>\g{</tt><i>n</i><tt>}</tt> for the <i>n</i><sup>th</sup><a href="#cg">capturing group</a> and <tt>\g{</tt><i>name</i><tt>}</tt> for <a href="#groupname">named-capturing group</a>.</p></li>
/// <li>
/// <p>The named character construct, <tt>\N{</tt><i>name</i><tt>}</tt> for a Unicode character by its name.</p></li>
/// <li>
/// <p>The conditional constructs <tt>(?(</tt><i>condition</i><tt>)</tt><i>X</i><tt>)</tt> and <tt>(?(</tt><i>condition</i><tt>)</tt><i>X</i><tt>|</tt><i>Y</i><tt>)</tt>,</p></li>
/// <li>
/// <p>The embedded code constructs <tt>(?{</tt><i>code</i><tt>})</tt> and <tt>(??{</tt><i>code</i><tt>})</tt>,</p></li>
/// <li>
/// <p>The embedded comment syntax <tt>(?#comment)</tt>, and</p></li>
/// <li>
/// <p>The preprocessing operations <tt>\l</tt> <tt>\u</tt>, <tt>\L</tt>, and <tt>\U</tt>.</p></li>
/// </ul>
/// <p>Constructs supported by this class but not by Perl:</p>
/// <ul>
/// <li>
/// <p>Character-class union and intersection as described <a href="#cc">above</a>.</p></li>
/// </ul>
/// <p>Notable differences from Perl:</p>
/// <ul>
/// <li>
/// <p>In Perl, <tt>\1</tt> through <tt>\9</tt> are always interpreted as back references; a backslash-escaped number greater than <tt>9</tt> is treated as a back reference if at least that many subexpressions exist, otherwise it is interpreted, if possible, as an octal escape. In this class octal escapes must always begin with a zero. In this class, <tt>\1</tt> through <tt>\9</tt> are always interpreted as back references, and a larger number is accepted as a back reference if at least that many subexpressions exist at that point in the regular expression, otherwise the parser will drop digits until the number is smaller or equal to the existing number of groups or it is one digit.</p></li>
/// <li>
/// <p>Perl uses the <tt>g</tt> flag to request a match that resumes where the last match left off. This functionality is provided implicitly by the <a href="../../../java/util/regex/Matcher.html" title="class in java.util.regex"><code>Matcher</code></a> class: Repeated invocations of the <a href="../../../java/util/regex/Matcher.html#find--"><code>find</code></a> method will resume where the last match left off, unless the matcher is reset.</p></li>
/// <li>
/// <p>In Perl, embedded flags at the top level of an expression affect the whole expression. In this class, embedded flags always take effect at the point at which they appear, whether they are at the top level or within a group; in the latter case, flags are restored at the end of the group just as in Perl.</p></li>
/// </ul>
/// <p>For a more precise description of the behavior of regular expression constructs, please see <a href="http://www.oreilly.com/catalog/regex3/"> <i>Mastering Regular Expressions, 3nd Edition</i>, Jeffrey E. F. Friedl, O'Reilly and Associates, 2006.</a></p>
pub struct JavaPattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaPattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPattern<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaPattern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/regex/Pattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn pattern(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pattern", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn quote(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("Javajava/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "quote",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn as_predicate(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "asPredicate",
            "()Ljava/util/function/Predicate;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn as_match_predicate(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "asMatchPredicate",
            "()Ljava/util/function/Predicate;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn split_as_stream(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "splitAsStream",
            "(Ljava/lang/CharSequence;)Ljava/util/stream/Stream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn flags(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "flags", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn matches(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let val_2 = arg1;
        let cls = jni.find_class("Javaboolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compile_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(
            jni.new_string(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("Javajava/util/regex/Pattern");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn matcher(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
