using System;
using Deveel;

namespace csmal
{
    public class Step3Env
    {
        private readonly Environment _env = Environment.Make(null);
        
        private Step3Env()
        {
            
        }

        // ReSharper disable once UnusedParameter.Local
        private static void Main(string[] args)
        {
            Step3Env step3Env = Make();
            while (true)
            {
                var line = Readline.ReadLine("REPL> ");
                if (line == null)
                {
                    break;
                }
                try
                {
                    Console.WriteLine(step3Env.Rep(line));
                }
                catch (Exception e)
                {
                    Console.WriteLine("ERROR: " + e.Message);
                }
            }
        }

        private MalType Read(string arg)
        {
            return Reader.ReadStr(arg);
        }

        private string Print(MalType arg)
        {
            return Printer.PrStr(arg);
        }

        public string Rep(string arg)
        {
            try
            {
                return Print(Evaluator.Eval(Read(arg), _env));
            }
            catch (NoTokensException)
            {
                return ""; // in this case, by design, we don't worry about the input
            }
        }

        public static Step3Env Make()
        {
            return new Step3Env();
        }

        public Func<string, string> GetRep()
        {
            return Rep;
        }
    }
}