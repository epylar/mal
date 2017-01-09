using System;
using System.Collections.Generic;
using System.Linq;

namespace csmal
{
    internal class MalQuote : MalType
    {
        private readonly MalType _val;

        public MalQuote(MalType input)
        {
            _val = input;
        }

        public override string ToString()
        {
            return "(quote " + _val + ")";
        }
    }

    internal class MalSpliceUnquote : MalType
    {
        private readonly MalType _val;

        public MalSpliceUnquote(MalType input)
        {
            _val = input;
        }

        public override string ToString()
        {
            return "(splice-unquote " + _val + ")";
        }
    }

    internal class MalMetaData : MalType
    {
        private readonly MalType _a;
        private readonly MalType _b;

        public MalMetaData(MalType a, MalType b)
        {
            _a = a;
            _b = b;
        }

        public override string ToString()
        {
            return "(with-meta " + _a + " " +_b + ")";
        }
    }

    internal class MalDeref : MalType
    {
        private readonly MalType _val;

        public MalDeref(MalType val)
        {
            _val = val;
        }

        public override string ToString()
        {
            return "(deref " + _val + ")";
        }
    }

    public abstract class MalType
    {
        public abstract override string ToString();
    }

    public class MalSymbol : MalType, IEquatable<MalSymbol>
    {
        private readonly string _val;

        public bool Equals(MalSymbol other)
        {
            if (ReferenceEquals(null, other)) return false;
            if (ReferenceEquals(this, other)) return true;
            return string.Equals(_val, other._val);
        }

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(null, obj)) return false;
            if (ReferenceEquals(this, obj)) return true;
            if (obj.GetType() != GetType()) return false;
            return Equals((MalSymbol) obj);
        }

        public override int GetHashCode()
        {
            return _val.GetHashCode();
        }

        public static bool operator ==(MalSymbol left, MalSymbol right)
        {
            return Equals(left, right);
        }

        public static bool operator !=(MalSymbol left, MalSymbol right)
        {
            return !Equals(left, right);
        }

        public MalSymbol(string symbol)
        {
            _val = symbol;
        }

        public override string ToString()
        {
            return _val;
        }
    }

    public abstract class AbstractMalListlike<T> : MalType where T : MalType
    {
        protected List<T> List; 

        public List<T> GetItems()
        {
            return List;
        }

        public abstract MalType Repackage(List<T> input);
    }

    public class MalList<T> : AbstractMalListlike<T> where T : MalType
    {
        public MalList(List<T> list)
        {
            List = list;
        }

        public override MalType Repackage(List<T> input)
        {
            return new MalList<T>(input);
        }

        public override string ToString()
        {
            return "(" + List.Select(x => x.ToString()).Aggregate((stuff, newthing) => stuff + " " + newthing) + ")";
        }
    }

    public class MalLong : MalType, IEquatable<MalLong>
    {
        public bool Equals(MalLong other)
        {
            if (ReferenceEquals(null, other)) return false;
            if (ReferenceEquals(this, other)) return true;
            return _value == other._value;
        }

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(null, obj)) return false;
            if (ReferenceEquals(this, obj)) return true;
            if (obj.GetType() != this.GetType()) return false;
            return Equals((MalLong) obj);
        }

        public override int GetHashCode()
        {
            return _value.GetHashCode();
        }

        public static bool operator ==(MalLong left, MalLong right)
        {
            return Equals(left, right);
        }

        public static bool operator !=(MalLong left, MalLong right)
        {
            return !Equals(left, right);
        }

        private readonly long _value;

        private MalLong(long value)
        {
            _value = value;
        }

        public static MalType Of(long value)
        {
            return new MalLong(value);
        }

        public static explicit operator long(MalLong malLong)
        {
            return malLong._value;
        }

        public static MalLong operator +(MalLong a, MalLong b)
        {
            return new MalLong(a._value + b._value);
        }

        public override string ToString()
        {
            return _value.ToString();
        }
    }

    public class MalHashMap<TX, TY> : MalType where TX : MalType where TY : MalType
    {
        private readonly Dictionary<TX, TY> _map;

        public MalHashMap(Dictionary<TX, TY> elements)
        {
            _map = elements;
        }

        public Dictionary<TX, TY> GetItems()
        {
            return _map;
        }

        public MalHashMap<TX, TY> Repackage(Dictionary<TX, TY> input)
        {
            return new MalHashMap<TX, TY>(input);
        }

        public override string ToString()
        {
            return "{" + _map.Select(x => x.Key + " " + x.Value).Aggregate((stuff, newstuff) => stuff + " " + newstuff) +
                   "}";
        }
    }

    internal class MalVector<T> : AbstractMalListlike<T> where T : MalType
    {
        public MalVector(List<T> elements)
        {
            List = elements;
        }

        public override string ToString()
        {
            return "[" + List.Select(x => x.ToString()).Aggregate((stuff, morestuff) => stuff + " " + morestuff) + "]";
        }

        public override MalType Repackage(List<T> input)
        {
            return new MalVector<T>(input);
        }
    }

    internal class MalUnquote : MalType
    {
        private readonly MalType _val;
        public MalUnquote(MalType val)
        {
            _val = val;
        }

        public override string ToString()
        {
            return "(unquote " + _val + ")";
        }
    }

    internal class MalQuasiQuote : MalType
    {
        private readonly MalType _val;
        public MalQuasiQuote(MalType val)
        {
            _val = val;
        }

        public override string ToString()
        {
            return "(quasiquote " + _val + ")";
        }
    }
}