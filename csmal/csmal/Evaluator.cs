using System;
using System.Collections.Generic;
using System.Linq;

namespace csmal
{
    static public class Evaluator
    {
        public static MalType Eval(MalType arg, Environment environment)
        {
            var argList = arg as MalList<MalType>;
            bool breakHere = (arg.ToString() == "(+ 7 8)");
            if (argList != null)
            {
                if (argList.GetItems()[0].Equals(new MalSymbol("def!")))
                {
                    return environment.Set(argList.GetItems()[1] as MalSymbol, Eval(argList.GetItems()[2], environment));
                }

                if (argList.GetItems()[0].Equals(new MalSymbol("let*")))
                {
                    var letEnv = Environment.Make(environment);
                    var listLike = argList.GetItems()[1] as AbstractMalListlike<MalType>;
                    if (listLike != null)
                    {
                        var letParams = listLike.GetItems();
                        int i = 0;
                        do
                        {
                            var key = letParams[i] as MalSymbol;
                            var value = Eval(letParams[i + 1], letEnv);
                            letEnv.Set(key, value);
                            i = i + 2;
                        } while (i < letParams.Count);

                    }

                    return Eval(argList.GetItems()[2], letEnv);
                }

                MalList<MalType> newEvaluatedList = ((MalList<MalType>)eval_ast(arg, environment));
                var items = newEvaluatedList.GetItems();
                var firstItem = items.First() as MalSymbol;
                var restItems = new MalList<MalType>(items.GetRange(1, items.Count - 1));

                var application = environment.Apply(firstItem, restItems);
                Console.WriteLine("Result of apply: " + application);
                return application;
            }
            /*
             * ast is not a list: then return the result of calling eval_ast on it.
             */
            return eval_ast(arg, environment);
        }

        internal static MalType eval_ast(MalType ast, Environment environment)
        {
            var symbol = ast as MalSymbol;
            if (symbol != null)
            {
                if (environment.HasSymbol(symbol))
                {
                    return environment.GetSymbol(symbol);
                }
                return symbol;
            }

            var hash = ast as MalHashMap<MalType, MalType>;
            if (hash != null)
            {
                var items = hash.GetItems();
                Dictionary<MalType, MalType> output = items.Keys.ToDictionary(key => key, key => Eval(items[key], environment));

                return hash.Repackage(output);
            }

            var listlike = ast as AbstractMalListlike<MalType>;
            if (listlike != null)
            {
                List<MalType> output = listlike.GetItems().Select(item => Eval(item, environment)).ToList();

                return listlike.Repackage(output);
            }

            return ast;
        }
    }
}