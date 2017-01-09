using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace csmal
{
    public class Environment
    {
        private readonly Dictionary<MalSymbol, Func<MalList<MalType>, MalType>> _funcs;
        private readonly Dictionary<MalSymbol, MalType> _dict;
        private readonly Environment _outer;

        private Environment(Dictionary<MalSymbol, Func<MalList<MalType>, MalType>> funcs, Environment outer)
        {
            _funcs = funcs;
            _dict = new Dictionary<MalSymbol, MalType>();
            _outer = outer;
        }

        public static Environment Make(Environment outer)
        {
            return new Environment(new Dictionary
                <MalSymbol, Func<MalList<MalType>, MalType>>
            {
                {new MalSymbol("+"), Plus},
                {new MalSymbol("-"), Minus},
                {new MalSymbol("*"), Times},
                {new MalSymbol("/"), IntegerDivide}
            }, outer);
        }

        private static MalType IntegerDivide(MalList<MalType> arg)
        {
            var items = arg.GetItems().Select(x => (long)(MalLong)x).ToImmutableList();
            if (items.Count() != 2)
            {
                throw new WrongNumberOfArgumentsException("wrong number of arguments for divide", null);
            }
            return MalLong.Of(items[0]/items[1]);
        }

        private static MalType Times(MalList<MalType> arg)
        {
            var items = arg.GetItems().Select(x => (long) (MalLong) x);
            long result = items.Aggregate<long, long>(1, (current, item) => current*item);
            return MalLong.Of(result);
        }

        private static MalType Minus(MalList<MalType> arg)
        {
            var items = arg.GetItems().Select(x => (long) (MalLong) x).ToList();
            if (items.Count() != 2)
            {
                throw new WrongNumberOfArgumentsException("wrong number of arguments for minus", null);
            }
            return MalLong.Of(items[0] - items[1]);
        }

        private static MalType Plus(MalList<MalType> arg)
        {

            return arg.GetItems().Aggregate(MalLong.Of(0), (current, item) => (MalLong) current + (MalLong) item);
        }

        internal bool HasSymbol(MalSymbol symbol)
        {
            return DictHasSymbol(symbol) || FuncsHasSymbol(symbol) || (_outer != null && _outer.HasSymbol(symbol));
        }

        internal bool DictHasSymbol(MalSymbol symbol)
        {
            return _dict.ContainsKey(symbol);            
        }

        internal bool FuncsHasSymbol(MalSymbol symbol)
        {
            return _funcs.ContainsKey(symbol);
        }

        internal MalType GetSymbol(MalSymbol symbol)
        {
            if (DictHasSymbol(symbol))
            {
                return _dict[symbol];
            }
            if (FuncsHasSymbol(symbol))
            {
                return symbol;
            }
            if (_outer != null && _outer.HasSymbol(symbol))
            {
                return _outer.GetSymbol(symbol);
            }
            throw new SymbolNotFoundException("attempted to Environment with invalid symbol", null);
        }

        public MalType Apply(MalSymbol firstItem, MalList<MalType> restItems)
        {
            if (FuncsHasSymbol(firstItem))
            {
                return _funcs[firstItem](restItems);
            }
            
            if (_outer != null)
            {
                return _outer.Apply(firstItem, restItems);
            }

            return null;
        }

        public MalType Set(MalSymbol key, MalType value)
        {            
            _dict[key] = value;
            return value;
        }

        public MalType Get(MalSymbol key)
        {
            if (DictHasSymbol(key))
            {
                return _dict[key];
            }

            if (_outer != null)
            {
                return _outer.Get(key);
            }

            return null;
        }

        public override string ToString()
        {
            return "Environment with " + _funcs.Count + " funcs and " + _dict.Count + " entries";
        }
    }
}