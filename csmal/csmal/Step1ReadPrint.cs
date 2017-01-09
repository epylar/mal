using System;
using Deveel;

namespace csmal
{
    public class Step1ReadPrint
    {
        private static void Main(string[] args)
        {
            while (true)
            {
                var line = Readline.ReadLine("REPL> ");
                if (line == null)
                {
                    break;
                }
                Console.WriteLine(Rep(line));
            }
        }

        private static MalType Read(string arg)
        {
            return Reader.ReadStr(arg);
        }

        private static MalType Eval(MalType arg)
        {
            return arg;
        }

        private static string Print(MalType arg)
        {
            return Printer.PrStr(arg);
        }

        public static string Rep(string arg)
        {
            try
            {
                return Print(Eval(Read(arg)));
            }
            catch (NoTokensException)
            {
                return ""; // in this case, by design, we don't worry about the input
            }
        }
    }
}