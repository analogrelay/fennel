namespace System;
/* By default, attributes are inherited and multiple attributes are not allowed */
[AttributeUsage(AttributeTargets.Class, Inherited = true)]
public sealed class AttributeUsageAttribute : Attribute
{
    public bool AllowMultiple { get; set; }

    public bool Inherited { get; set; }

    public AttributeUsageAttribute(AttributeTargets validOn)
    {
    }

    public AttributeUsageAttribute(AttributeTargets validOn, bool allowMultiple, bool inherited)
    {
    }
}