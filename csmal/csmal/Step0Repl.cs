using System;
using Deveel;

namespace csmal
{
    public class Step0Repl
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

        private static string Read(string arg)
        {
            return arg;
        }

        private static string Eval(string arg)
        {
            return arg;
        }

        private static string Print(string arg)
        {
            return arg;
        }

        public static string Rep(string arg)
        {
            return Print(Eval(Read(arg)));
        }
    }
}