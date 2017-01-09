<<<<<<< HEAD
using System;
=======
﻿using System;
>>>>>>> origin/master
using System.Collections.Generic;
using System.Linq;

namespace csmalUnitTests
{
    public class Case : List<Step>
    {
        public override string ToString()
        {
            return "Test case: " + String.Join("\n", this.Select(x => x.ToString()));
        }
    }

    public class Step
    {
        public List<string> Input { get; set; }
        public string Output { get; set; }
        public override string ToString()
        {
            return "[Step; Inputs: {" + String.Join("\n", Input) + "; Output: " +  Output + "]";
        }
    }
}