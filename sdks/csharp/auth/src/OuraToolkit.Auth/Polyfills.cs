#if NETSTANDARD2_0
// Compiler-required attribute polyfills so C# 9 records, `init`-only setters, and C# 11
// `required` members compile on netstandard2.0 (whose reference assemblies predate them).
// The standard minimal manual set — ZERO runtime dependency, no PolySharp. On net8/net10 the
// BCL already ships these types, so the whole file is fenced out to avoid duplicate
// definitions. Kept `internal` so they never leak into the public surface.
namespace System.Runtime.CompilerServices
{
    using System;

    /// <summary>Marker enabling <c>init</c>-only setters on netstandard2.0.</summary>
    internal static class IsExternalInit
    {
    }

    /// <summary>Marks a member as C# 11 <c>required</c> on netstandard2.0.</summary>
    [AttributeUsage(
        AttributeTargets.Field | AttributeTargets.Property,
        AllowMultiple = false,
        Inherited = false)]
    internal sealed class RequiredMemberAttribute : Attribute
    {
    }

    /// <summary>
    /// Signals that a compiler feature (e.g. required members) is needed to consume the
    /// annotated element; emitted by the compiler alongside <c>required</c> members.
    /// </summary>
    [AttributeUsage(AttributeTargets.All, AllowMultiple = true, Inherited = false)]
    internal sealed class CompilerFeatureRequiredAttribute : Attribute
    {
        /// <summary>Creates the attribute for the named feature.</summary>
        public CompilerFeatureRequiredAttribute(string featureName) => FeatureName = featureName;

        /// <summary>The required feature name.</summary>
        public string FeatureName { get; }

        /// <summary>Whether the feature is optional for older compilers.</summary>
        public bool IsOptional { get; init; }

        /// <summary>The <c>ref struct</c> feature name.</summary>
        public const string RefStructs = nameof(RefStructs);

        /// <summary>The required-members feature name.</summary>
        public const string RequiredMembers = nameof(RequiredMembers);
    }
}

namespace System.Diagnostics.CodeAnalysis
{
    using System;

    /// <summary>
    /// Tells the compiler a constructor initializes all <c>required</c> members, so callers
    /// need not use an object initializer.
    /// </summary>
    [AttributeUsage(AttributeTargets.Constructor, AllowMultiple = false, Inherited = false)]
    internal sealed class SetsRequiredMembersAttribute : Attribute
    {
    }
}
#endif
