using System;
using System.Collections.Generic;
using NUnit.Framework;

namespace csmalUnitTests
{
    static class CaseChecker
    {
        public static void CheckCases(List<Case> cases, Func<string, string> func)
        {
            foreach (Case @case in cases)
            {
                foreach (Step step in @case)
                {
                    Console.WriteLine("Checking: " + step);
                    string output = "";
                    foreach (var line in step.Input)
                    {
                        output = func.Invoke(line);
                    }

                    Assert.That(output, Is.EqualTo(step.Output));
                    Console.WriteLine("Success!");
                }
            }
        }

        public static void CheckCase(Case theCase, Func<string, string> rep)
        {
            CheckCases(new List<Case> {theCase}, rep);
        }
    }
}
