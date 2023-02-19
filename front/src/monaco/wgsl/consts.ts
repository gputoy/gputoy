import type * as monaco from 'monaco-editor'

const DETAIL_KEYWORD = 'wgsl keyword'
const DETAIL_ATTRIBUTE = 'wgsl attribute'

export const COMPLETION_KEYWORDS: monaco.languages.CompletionItem[] = [
	'alias',
	'bitcast',
	'break',
	'case',
	'const',
	'const_assert',
	'continue',
	'continuing',
	'default',
	'discard',
	'else',
	'enable',
	'false',
	'fn',
	'for',
	'if',
	'let',
	'loop',
	'override',
	'return',
	'struct',
	'switch',
	'true',
	'var',
	'while'
].map((keyword) => ({
	insertText: keyword,
	kind: 17, //monaco.languages.CompletionItemKind.Keyword
	label: keyword,
	documentation: {
		value:
			'<b>let<\b>\nBind a new variable to identifier.\n Immutable.\n```wgsl\nlet val = vec3<f32>(0.);\nval.x;\n```',
		supportHtml: true
	},
	detail: 'wgsl keyword',
	range: undefined as unknown as monaco.IRange
}))

export const COMPLETION_ATTRIBUTES: monaco.languages.CompletionItem[] = [
	'align',
	'binding',
	'builtin',
	'compute',
	'const',
	'fragment',
	'group',
	'id',
	'interpolate',
	'invariant',
	'location',
	'size',
	'vertex',
	'workgroup_size'
]
	.map((attribute) => ({
		insertText: attribute,
		kind: 5, //monaco.languages.CompletionItemKind.Class
		detail: 'wgsl attribute',
		label: attribute,
		range: undefined as unknown as monaco.IRange
	}))
	.concat([
		{
			insertText: 'export',
			kind: 5,
			detail: 'gputoy attribute',
			label: 'export',
			range: undefined as unknown as monaco.IRange
		},
		{
			insertText: 'import',
			kind: 5,
			detail: 'gputoy attribute',
			label: 'import',
			range: undefined as unknown as monaco.IRange
		}
	])

export const COMPLETION_BUILTIN_VALUE: readonly monaco.languages.CompletionItem[] =
	[].map((keyword) => ({
		insertText: keyword,
		kind: 17, //monaco.languages.CompletionItemKind.Keyword
		label: keyword,
		range: undefined as unknown as monaco.IRange
	}))

export const COMPLETIONS_BUILTIN_TYPES: monaco.languages.CompletionItem[] = [
	['u32', 'unsigned integer'],
	['i32', 'signed integer'],
	['f32', 'float']
]
	.flatMap(([ty, desc]) => [
		{
			label: ty,
			detail: `32-bit ${desc}`
		},
		{
			label: `vec2<${ty}>`,
			detail: `Two 32-bit ${desc}s`
		},
		{
			label: `vec3<${ty}>`,
			detail: `Three 32-bit ${desc}s`
		},
		{
			label: `vec4<${ty}>`,
			detail: `Four 32-bit ${desc}s`
		}
	])
	.map((ty) => ({
		insertText: ty.label,
		kind: 24, //monaco.languages.CompletionItemKind.TypeParameter
		// kind: 17,
		range: undefined as unknown as monaco.IRange,
		...ty
	}))

export const COMPLETIONS_BUILTIN_FUNCTIONS: monaco.languages.CompletionItem[] =
	[
		{
			detail: 'abs(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the absolute value of e (e.g. e with a positive sign bit). Component-wise when T is a vector.\n (GLSLstd450Fabs)',
			label: 'abs',
			insertText: 'abs(${1})'
		},
		{
			detail: 'acos(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the arc cosine of e. Component-wise when T is a vector.\n (GLSLstd450Acos)',
			label: 'acos',
			insertText: 'acos(${1})'
		},
		{
			detail: 'asin(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the arc sine of e. Component-wise when T is a vector.\n (GLSLstd450Asin)',
			label: 'asin',
			insertText: 'asin(${1})'
		},
		{
			detail: 'atan(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the arc tangent of e. Component-wise when T is a vector.\n (GLSLstd450Atan)',
			label: 'atan',
			insertText: 'atan(${1})'
		},
		{
			detail: 'atan2(e1: T, e2: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the arc tangent of e1 over e2. Component-wise when T is a vector.\n (GLSLstd450Atan2)',
			label: 'atan2',
			insertText: 'atan2(${1})'
		},
		{
			detail: 'ceil(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the ceiling of e. Component-wise when T is a vector.\n (GLSLstd450Ceil)',
			label: 'ceil',
			insertText: 'ceil(${1})'
		},
		{
			detail: 'clamp(e1: T,  e2: T,  e3: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns min(max(e1,e2),e3). Component-wise when T is a vector.\n (GLSLstd450NClamp)',
			label: 'clamp',
			insertText: 'clamp(${1})'
		},
		{
			detail: 'cos(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the cosine of e. Component-wise when T is a vector.\n (GLSLstd450Cos)',
			label: 'cos',
			insertText: 'cos(${1})'
		},
		{
			detail: 'cosh(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the hyperbolic cosine of e. Component-wise when T is a vector\n (GLSLstd450Cosh)',
			label: 'cosh',
			insertText: 'cosh(${1})'
		},
		{
			detail: 'cross(e1: vec3<T> ,e2: vec3<T>) -> vec3<T> where: T is f32',
			documentation:
				'Returns the cross product of e1 and e2. (GLSLstd450Cross)',
			label: 'cross',
			insertText: 'cross(${1})'
		},
		{
			detail: 'degrees(e1: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Converts radians to degrees, approximating e1×180÷π. Component-wise when T is a vector (GLSLstd450Degrees)',
			label: 'degrees',
			insertText: 'degrees(${1})'
		},
		{
			detail: 'distance(e1: T, e2: T) -> f32 where: T is f32 or vecN<f32>',
			documentation:
				'Returns the distance between e1 and e2 (e.g. length(e1-e2)).\n (GLSLstd450Distance)',
			label: 'distance',
			insertText: 'distance(${1})'
		},
		{
			detail: 'exp(e1: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the natural exponentiation of e1 (e.g. ee1). Component-wise when T is a vector.\n (GLSLstd450Exp)',
			label: 'exp',
			insertText: 'exp(${1})'
		},
		{
			detail: 'exp2(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns 2 raised to the power e (e.g. 2e). Component-wise when T is a vector.\n (GLSLstd450Exp2)',
			label: 'exp2',
			insertText: 'exp2(${1})'
		},
		{
			detail: 'faceForward(e1: T, e2: T, e3: T) -> T where: T is vecN<f32>',
			documentation:
				'Returns e1 if dot(e2,e3) is negative, and -e1 otherwise.\n (GLSLstd450FaceForward)',
			label: 'faceForward',
			insertText: 'faceForward(${1})'
		},
		{
			detail: 'floor(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the floor of e. Component-wise when T is a vector.\n (GLSLstd450Floor)',
			label: 'floor',
			insertText: 'floor(${1})'
		},
		{
			detail: 'fma(e1: T, e2: T, e3: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns e1 * e2 + e3. Component-wise when T is a vector.\n (GLSLstd450Fma)',
			label: 'fma',
			insertText: 'fma(${1})'
		},
		{
			detail: 'fract(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the fractional bits of e (e.g. e - floor(e)). Component-wise when T is a vector.\n (GLSLstd450Fract)',
			label: 'fract',
			insertText: 'fract(${1})'
		},
		{
			detail: 'frexp(e:T) -> __frexp_result where: T is f32',
			documentation:
				'Splits e into a significand and exponent of the form significand * 2exponent.\n Returns the __frexp_result built-in structure, defined as if as follows: \nstruct __frexp_result {\nsig : f32; // significand part\nexp : i32; // exponent part\n};\n\n     The magnitude of the significand is in the range of [0.5, 1.0) or 0. \n    Note: A value cannot be explicitly declared with the type __frexp_resulT,  but a value may infer the type.\n\n\nEXAMPLE: frexp usage\n// Infers result type\nlet sig_and_exp = frexp(1.5);\n// Sets fraction_direct to 0.75\nlet fraction_direct = frexp(1.5).sig;\n\n\n(GLSLstd450FrexpStruct)',
			label: 'frexp',
			insertText: 'frexp(${1})'
		},
		{
			detail: 'frexp(e:T) -> __frexp_result_vecN where: T is vecN<f32>',
			documentation:
				'Splits the components of e into a significand and exponent of the form significand * 2exponent.\n Returns the __frexp_result_vecN built-in structure, defined as if as follows: \nstruct __frexp_result_vecN {\nsig : vecN<f32>; // significand part\nexp : vecN<i32>; // exponent part\n};\n\n     The magnitude of each component of the significand is in the range of [0.5, 1.0) or 0. \n    Note: A value cannot be explicitly declared with the type __frexp_result_vecN, but a value may infer the type.\n(GLSLstd450FrexpStruct)',
			label: 'frexp',
			insertText: 'frexp(${1})'
		},
		{
			detail: 'inverseSqrt(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the reciprocal of sqrt(e). Component-wise when T is a vector.\n (GLSLstd450InverseSqrt)',
			label: 'inverseSqrt',
			insertText: 'inverseSqrt(${1})'
		},
		{
			detail:
				'ldexp(e1: T, e2: I ) -> T where: T is f32 or vecN<f32> I is i32 or vecN<i32>, where I is a scalar if T is a scalar, or a vector when T is a vector',
			documentation:
				'Returns e1 * 2e2. Component-wise when T is a vector.\n (GLSLstd450Ldexp)',
			label: 'ldexp',
			insertText: 'ldexp(${1})'
		},
		{
			detail: 'length(e: T) -> f32 where: T is f32 or vecN<f32>',
			documentation:
				'Returns the length of e (e.g. abs(e) if T is a scalar, or sqrt(e[0]2 + e[1]2 + ...) if T is a vector).\n (GLSLstd450Length)',
			label: 'length',
			insertText: 'length(${1})'
		},
		{
			detail: 'log(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the natural logarithm of e. Component-wise when T is a vector.\n (GLSLstd450Log)',
			label: 'log',
			insertText: 'log(${1})'
		},
		{
			detail: 'log2(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the base-2 logarithm of e. Component-wise when T is a vector.\n (GLSLstd450Log2)',
			label: 'log2',
			insertText: 'log2(${1})'
		},
		{
			detail: 'max(e1: T, e2: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns e2 if e1 is less than e2, and e1 otherwise.\n If one operand is a NaN, the other is returned.\n If both operands are NaNs, a NaN is returned. Component-wise when T is a vector.\n (GLSLstd450NMax)',
			label: 'max',
			insertText: 'max(${1})'
		},
		{
			detail: 'min(e1: T, e2: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns e2 if e2 is less than e1, and e1 otherwise.\n If one operand is a NaN, the other is returned.\n If both operands are NaNs, a NaN is returned. Component-wise when T is a vector.\n (GLSLstd450NMin)',
			label: 'min',
			insertText: 'min(${1})'
		},
		{
			detail: 'mix(e1: T, e2: T, e3: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the linear blend of e1 and e2 (e.g. e1*(1-e3)+e2*e3). Component-wise when T is a vector.  (GLSLstd450FMix)',
			label: 'mix',
			insertText: 'mix(${1})'
		},
		{
			detail: 'mix(e1: T, e2: T,  e3: f32 ) -> T where: T is vecN<f32>',
			documentation:
				'Returns the component-wise linear blend of e1 and e2,\n     using scalar blending factor e3 for each component. Same as mix(e1,e2,T(e3)).',
			label: 'mix',
			insertText: 'mix(${1})'
		},
		{
			detail: 'modf(e:T) -> __modf_result where: T is f32',
			documentation:
				'Splits e into fractional and whole number parts.\n Returns the __modf_result built-in structure, defined as if as follows: \nstruct __modf_result {\nfract : f32; // fractional part\nwhole : f32; // whole part\n};\n\nNote: A value cannot be explicitly declared with the type __modf_resulT,   but a value may infer the type.\n\n\nEXAMPLE: modf usage\n// Infers result type\nlet fract_and_whole = modf(1.5);\n// Sets fract_direct to 0.5\nlet fract_direct = modf(1.5).fract;\n\n\n(GLSLstd450ModfStruct)',
			label: 'modf',
			insertText: 'modf(${1})'
		},
		{
			detail: 'modf(e:T) -> __modf_result_vecN where: T is vecN<f32>',
			documentation:
				'Splits the components of e into fractional and whole number parts.\n Returns the __modf_result_vecN built-in structure, defined as if as follows: \nstruct __modf_result_vecN {\nfract : vecN<f32>; // fractional part\nwhole : vecN<f32>; // whole part\n};\n\nNote: A value cannot be explicitly declared with the type __modf_result_vecN, but a value may infer the type.\n(GLSLstd450ModfStruct)',
			label: 'modf',
			insertText: 'modf(${1})'
		},
		{
			detail: 'normalize(e: vecN<T> ) -> vecN<T> where: T is f32',
			documentation:
				'Returns a unit vector in the same direction as e.\n (GLSLstd450Normalize)',
			label: 'normalize',
			insertText: 'normalize(${1})'
		},
		{
			detail: 'pow(e1: T,  e2: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns e1 raised to the power e2. Component-wise when T is a vector.\n (GLSLstd450Pow)',
			label: 'pow',
			insertText: 'pow(${1})'
		},
		{
			detail: 'quantizeToF16(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Quantizes a 32-bit floating point value e as if e were converted to a IEEE 754 binary16 value,\n     and then converted back to a IEEE 754 binary32 value. See \u00c2\u00a7\u00e2\u20ac\u00af12.5.2 Floating point conversion. Component-wise when T is a vector.\nNote: The vec2<f32> case is the same as unpack2x16float(pack2x16float(e)).\n(OpQuantizeToF16)',
			label: 'quantizeToF16',
			insertText: 'quantizeToF16(${1})'
		},
		{
			detail: 'radians(e1: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Converts degrees to radians, approximating e1\u00a0\u00c3\u2014\u00a0\u00cf\u20ac\u00a0\u00c3\u00b7\u00a0180. Component-wise when T is a vector (GLSLstd450Radians)',
			label: 'radians',
			insertText: 'radians(${1})'
		},
		{
			detail: 'reflect(e1: T,  e2: T) -> T where: T is vecN<f32>',
			documentation:
				'For the incident vector e1 and surface orientation e2, returns the reflection direction e1-2*dot(e2,e1)*e2.\n (GLSLstd450Reflect)',
			label: 'reflect',
			insertText: 'reflect(${1})'
		},
		{
			detail:
				'refract(e1: T,  e2: T,  e3: I ) -> T where: T is vecN<f32>I is f32',
			documentation:
				'For the incident vector e1 and surface normal e2, and the ratio of indices of refraction e3,\n let k = 1.0 -e3*e3* (1.0 - dot(e2,e1) * dot(e2,e1)). If k < 0.0, returns the\n refraction vector 0.0, otherwise return the refraction vector e3*e1- (e3* dot(e2,e1) + sqrt(k)) *e2.\n (GLSLstd450Refract)',
			label: 'refract',
			insertText: 'refract(${1})'
		},
		{
			detail: 'round(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Result is the integer k nearest to e, as a floating point value. When e lies halfway between integers k and k+1,\n     the result is k when k is even, and k+1 when k is odd. Component-wise when T is a vector.\n     (GLSLstd450RoundEven)',
			label: 'round',
			insertText: 'round(${1})'
		},
		{
			detail: 'sign(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the sign of e. Component-wise when T is a vector.\n (GLSLstd450FSign)',
			label: 'sign',
			insertText: 'sign(${1})'
		},
		{
			detail: 'sin(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the sine of e. Component-wise when T is a vector.\n (GLSLstd450Sin)',
			label: 'sin',
			insertText: 'sin(${1})'
		},
		{
			detail: 'sinh(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the hyperbolic sine of e. Component-wise when T is a vector.\n (GLSLstd450Sinh)',
			label: 'sinh',
			insertText: 'sinh(${1})'
		},
		{
			detail:
				'smoothStep(e1: T,  e2: T,  e3: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the smooth Hermite interpolation between 0 and 1. Component-wise when T is a vector.\n (GLSLstd450SmoothStep)',
			label: 'smoothStep',
			insertText: 'smoothStep(${1})'
		},
		{
			detail: 'sqrt(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the square root of e. Component-wise when T is a vector.\n (GLSLstd450Sqrt)',
			label: 'sqrt',
			insertText: 'sqrt(${1})'
		},
		{
			detail: 'step(e1: T,  e2: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns 0.0 if e1 is less than e2, and 1.0 otherwise. Component-wise when T is a vector.\n (GLSLstd450Step)',
			label: 'step',
			insertText: 'step(${1})'
		},
		{
			detail: 'tan(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the tangent of e. Component-wise when T is a vector.\n (GLSLstd450Tan)',
			label: 'tan',
			insertText: 'tan(${1})'
		},
		{
			detail: 'tanh(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the hyperbolic tangent of e. Component-wise when T is a vector.\n (GLSLstd450Tanh)',
			label: 'tanh',
			insertText: 'tanh(${1})'
		},
		{
			detail: 'trunc(e: T) -> T where: T is f32 or vecN<f32>',
			documentation:
				'Returns the nearest whole number whose absolute value is less than or equal to e. Component-wise when T is a vector.\n (GLSLstd450Trunc)',
			label: 'trunc',
			insertText: 'trunc(${1})'
		},

		// INTEGER FUNCTIONS
		{
			detail: 'abs(e: T) -> T where: T is i32 or vecN<i32>',
			documentation:
				'The absolute value of e. Component-wise when T is a vector.\n     If e evaluates to the largest negative value, then the result is e.\n     (GLSLstd450SAbs)',
			label: 'abs',
			insertText: 'abs(${1})'
		},
		{
			detail: 'abs(e: T) -> T where: T is u32 or vecN<u32>',
			documentation:
				'Result is e.  This is provided for symmetry with abs for signed integers. Component-wise when T is a vector.',
			label: 'abs',
			insertText: 'abs(${1})'
		},
		{
			detail: 'clamp(e1: T,  e2: T, e3: T) -> T where: T is u32 or vecN<u32>',
			documentation:
				'Returns min(max(e1,e2),e3). Component-wise when T is a vector.\n (GLSLstd450UClamp)',
			label: 'clamp',
			insertText: 'clamp(${1})'
		},
		{
			detail: 'clamp(e1: T,  e2: T, e3: T) -> T where: T is i32 or vecN<i32>',
			documentation:
				'Returns min(max(e1,e2),e3). Component-wise when T is a vector.\n (GLSLstd450SClamp)',
			label: 'clamp',
			insertText: 'clamp(${1})'
		},
		{
			detail:
				'countOneBits(e: T) -> T where: T is i32, u32, vecN<i32>, or vecN<u32>',
			documentation:
				'The number of 1 bits in the representation of e. Also known as "population count". Component-wise when T is a vector.\n     (SPIR-V OpBitCount)',
			label: 'countOneBits',
			insertText: 'countOneBits(${1})'
		},
		{
			detail: 'max(e1: T, e2: T) -> T where: T is u32 or vecN<u32>',
			documentation:
				'Returns e2 if e1 is less than e2, and e1 otherwise. Component-wise when T is a vector.\n (GLSLstd450UMax)',
			label: 'max',
			insertText: 'max(${1})'
		},
		{
			detail: 'max(e1: T, e2: T) -> T where: T is i32 or vecN<i32>',
			documentation:
				'Returns e2 if e1 is less than e2, and e1 otherwise. Component-wise when T is a vector.\n (GLSLstd450SMax)',
			label: 'max',
			insertText: 'max(${1})'
		},
		{
			detail: 'min(e1: T, e2: T) -> T where: T is u32 or vecN<u32>',
			documentation:
				'Returns e1 if e1 is less than e2, and e2 otherwise. Component-wise when T is a vector.\n (GLSLstd450UMin)',
			label: 'min',
			insertText: 'min(${1})'
		},
		{
			detail: 'min(e1: T, e2: T) -> T where: T is i32 or vecN<i32>',
			documentation:
				'Returns e1 if e1 is less than e2, and e2 otherwise. Component-wise when T is a vector.\n (GLSLstd45SUMin)',
			label: 'min',
			insertText: 'min(${1})'
		},
		{
			detail:
				'reverseBits(e: T) -> T where: T is i32, u32, vecN<i32>, or vecN<u32>',
			documentation:
				'Reverses the bits in e:  The bit at position k of the result equals the\n     bit at position 31-k of e. Component-wise when T is a vector.\n     (SPIR-V OpBitReverse)',
			label: 'reverseBits',
			insertText: 'reverseBits(${1})'
		},

		// VECTOR FUNCTIONS
		{
			detail: 'dot(e1: vecN<T>,e2: vecN<T>) -> T where: T is f32',
			documentation: 'Returns the dot product of e1 and e2.\n (OpDot)',
			label: 'dot',
			insertText: 'dot(${1})'
		},
		{
			detail: 'dot(e1: vecN<T>,e2: vecN<T>) -> T where: T is i32',
			documentation:
				'Returns the dot product of e1 and e2.\n (SPV_KHR_integer_dot_product OpSDotKHR)',
			label: 'dot',
			insertText: 'dot(${1})'
		},
		{
			detail: 'dot(e1: vecN<T>,e2: vecN<T>) -> T where: T is u32',
			documentation:
				'Returns the dot product of e1 and e2.\n (SPV_KHR_integer_dot_product OpUDotKHR)',
			label: 'dot',
			insertText: 'dot(${1})'
		},
		// type constructors
		{
			detail: '32 bit float of x, y',
			label: 'vec2<f32>',
			insertText: 'vec2<f32>(${1})'
		},
		{
			detail: '32 bit integer of x, y',
			label: 'vec2<i32>',
			insertText: 'vec2<i32>(${1})'
		},
		{
			detail: '32 bit unsigned integer of x, y',
			label: 'vec2<u32>',
			insertText: 'vec2<u32>(${1})'
		},

		{
			detail: '32 bit float of x, y, z',
			label: 'vec3<f32>',
			insertText: 'vec3<f32>(${1})'
		},
		{
			detail: '32 bit integer of x, y, z',
			label: 'vec3<i32>',
			insertText: 'vec3<i32>(${1})'
		},
		{
			detail: '32 bit unsigned integer of x, y, z',
			label: 'vec3<u32>',
			insertText: 'vec3<u32>(${1})'
		},

		{
			detail: '32 bit float of x, y, z, w',
			label: 'vec4<f32>',
			insertText: 'vec4<f32>(${1})'
		},
		{
			detail: '32 bit integer of x, y, z, w',
			label: 'vec4<i32>',
			insertText: 'vec4<i32>(${1})'
		},
		{
			detail: '32 bit unsigned integer of x, y, z, w',
			label: 'vec4<u32>',
			insertText: 'vec4<u32>(${1})'
		}
		// TODO: add matrices
	].map((completion) => ({
		range: undefined as unknown as monaco.IRange,
		kind: 1, //monaco.languages.CompletionItemKind.Function
		insertTextRules: 4,
		...completion
	}))
