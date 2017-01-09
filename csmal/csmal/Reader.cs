using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace csmal
{
    public class Reader
    {
        private static readonly Regex MalTokenRegex =
            new Regex(@"[\s,]*(~@|[\[\]{}()'`~^@]|""(?:\\.|[^\\""])*""|;.*|[^\s\[\]{}('""`,;)]*)");

        private static readonly Regex MalIntegerRegex =
            new Regex(@"^\-?[0123456789]+$");

        private readonly List<string> _tokens = new List<string>();

        private Reader(IEnumerable<string> tokens)
        {
            _tokens.AddRange(tokens);
        }

        public static MalType ReadStr(string str)
        {
            List <string> tokens = Tokenizer(str);
            if (!tokens.Any())
            {
                throw new NoTokensException("no tokens on this line", null);
            }
            var reader = new Reader(Tokenizer(str));
            return reader.ReadForm();
        }

        private MalType ReadForm()
        {
            var firstToken = _tokens.First(); // TODO what if there are no tokens?
            switch (firstToken)
            {
                case "(":
                    return ReadList();
                case "[":
                    return ReadArray();
                case "{":
                    return ReadHashMap();
                case "\'":
                    return ReadQuote();
                case "`":
                    return ReadQuasiQuote();
                case "~":
                    return ReadUnquote();
                case "~@":
                    return ReadSpliceUnquote();
                case "^":
                    return ReadMetaData();
                case "@":
                    return ReadDeref();
                default:
                    return ReadAtom();
            }
        }

        private MalType ReadDeref()
        {
            _tokens.RemoveAt(0); // remove the @
            return new MalDeref(ReadForm());
        }

        private MalType ReadMetaData()
        {
            _tokens.RemoveAt(0); // remove the ^
            MalType meta = ReadForm();
            MalType item = ReadForm();
            return new MalMetaData(item, meta);
        }


        private MalType ReadSpliceUnquote()
        {
            _tokens.RemoveAt(0); // remove the ~@
            return new MalSpliceUnquote(ReadForm());
        }

        private MalType ReadQuote()
        {
            _tokens.RemoveAt(0); // remove the '
            return new MalQuote(ReadForm());
        }

        private MalType ReadQuasiQuote()
        {
            _tokens.RemoveAt(0); // remove the `
            return new MalQuasiQuote(ReadForm());
        }

        private MalType ReadUnquote()
        {
            _tokens.RemoveAt(0); // remove the ~
            return new MalUnquote(ReadForm());
        }

        private MalType ReadAtom()
        {
            string token = _tokens[0];

            _tokens.RemoveAt(0);
            if (MalIntegerRegex.IsMatch(token))
            {
                return MalLong.Of(Int64.Parse(token));
            }
            return new MalSymbol(token);
        }

        private MalType ReadList()
        {
            var elements = new List<MalType>();

            _tokens.RemoveAt(0); // remove the "("
            while (_tokens.First() != ")")
            {
                elements.Add(ReadForm());
            }
            _tokens.RemoveAt(0); // remove the ")"

            return new MalList<MalType>(elements);
        }

        private MalType ReadArray()
        {
            var elements = new List<MalType>();

            _tokens.RemoveAt(0); // remove the "["
            while (_tokens.First() != "]")
            {
                elements.Add(ReadForm());
            }
            _tokens.RemoveAt(0); // remove the "]"

            return new MalVector<MalType>(elements);
        }

        private MalType ReadHashMap()
        {
            var elements = new Dictionary<MalType, MalType>();

            _tokens.RemoveAt(0); // remove the "{"
            while (_tokens.First() != "}")
            {
                elements.Add(ReadForm(), ReadForm());
            }
            _tokens.RemoveAt(0); // remove the "}

            return new MalHashMap<MalType, MalType>(elements);
        }

        private static List<string> Tokenizer(string str)
        {
            var matches = MalTokenRegex.Matches(str);
            try
            {
                List<string> tokens = matches.Cast<Match>().Select(match => match.Groups[1].Value).
                    Where(value => !String.IsNullOrEmpty(value) && value[0] != ';').ToList();
                return tokens;
            }
            catch (IndexOutOfRangeException)
            {
                // in this case, it is okay to return an empty list
                return new List<string>();
            }
        }
    }
}