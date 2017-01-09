using System;
using System.Collections.Generic;
using System.Linq;
using Deveel;

namespace csmal
{
    public class Step2Eval
    {
        // ReSharper disable once UnusedParameter.Local
        private static void Main(string[] args)
        {
            while (true)
            {
                var line = Readline.ReadLine("REPL> ");
                if (line == null)
                {
                    break;
                }
                try
                {
                    Console.WriteLine(Rep(line));
                }
                catch (Exception e)
                {
                    Console.WriteLine("ERROR: " + e.Message);
                }
            }
        }

        private static MalType Read(string arg)
        {
            return Reader.ReadStr(arg);
        }

//        private static MalType Eval(MalType arg, Environment environment)
//        {
//            if (arg is MalList<MalType>)
//            {
//                MalList<MalType> newEvaluatedList = ((MalList<MalType>)eval_ast(arg, environment));
//                var items = newEvaluatedList.GetItems();
//                MalType firstItem = items.First();
//                var restItems = new MalList<MalType>(items.GetRange(1, items.Count - 1));
//                return environment.Apply((MalSymbol)firstItem, restItems);
//            }
//            /*
//             * ast is not a list: then return the result of calling eval_ast on it.
//             */
//            return eval_ast(arg, environment);
//        }

        private static string Print(MalType arg)
        {
            return Printer.PrStr(arg);
        }

        public static string Rep(string arg)
        {
            try
            {
                return Print(Evaluator.Eval(Read(arg), Environment.Make(null)));
            }
            catch (NoTokensException)
            {
                return ""; // in this case, by design, we don't worry about the input
            }
        }

//        internal static MalType eval_ast(MalType ast, Environment environment)
//        {
//            var symbol = ast as MalSymbol;
//            if (symbol != null)
//            {
//                if (environment.HasSymbol(symbol))
//                {
//                    return environment.GetSymbol(symbol);
//                }
//                throw new SymbolNotFoundException("tried to use symbol not in environment", null);
//            }
//
//            var hash = ast as MalHashMap<MalType, MalType>;
//            if (hash != null)
//            {
//                var items = hash.GetItems();
//                Dictionary<MalType, MalType> output = items.Keys.ToDictionary(key => key, key => Eval(items[key], environment));
//
//                return hash.Repackage(output);
//            }
//
//            var listlike = ast as AbstractMalListlike<MalType>;
//            if (listlike != null)
//            {
//                List<MalType> output = listlike.GetItems().Select(item => Eval(item, environment)).ToList();
//
//                return listlike.Repackage(output);
//            }
//
//            return ast;
//        }
    }
}