<%
    import re
    import yacg.model.model as model
    import yacg.templateHelper as templateHelper
    import yacg.model.modelFuncs as modelFuncs
    import yacg.util.stringUtils as stringUtils

    templateFile = 'rust_types.mako'
    templateVersion = '0.1.0'

    testMod = templateParameters.get('testMod',None)
    testModPath = templateParameters.get('testModPath',None)
    


    def printRustType(typeObj, isArray, isRequired, arrayDimensions = 1):
        ret = ''
        if typeObj is None:
            return '???'
        elif isinstance(typeObj, model.IntegerType):
            if typeObj.format is None or typeObj.format == model.IntegerTypeFormatEnum.INT32:
                ret = 'i32'
            else:
                ret = 'int'
        elif isinstance(typeObj, model.ObjectType):
            ret = '()'
        elif isinstance(typeObj, model.NumberType):
            if typeObj.format is None or typeObj.format != model.NumberTypeFormatEnum.DOUBLE:
                ret = 'f32'
            else:
                ret = 'f64'
        elif isinstance(typeObj, model.BooleanType):
            ret = 'bool'
        elif isinstance(typeObj, model.StringType):
            ret = 'String'
        elif isinstance(typeObj, model.BytesType):
            ret = 'byte'
        elif isinstance(typeObj, model.UuidType):
            ret = 'Uuid'
        elif isinstance(typeObj, model.EnumType):
            ret = typeObj.name
        elif isinstance(typeObj, model.DateType):
            ret = 'Date'
        elif isinstance(typeObj, model.TimeType):
            ret = 'String'
        elif isinstance(typeObj, model.DateTimeType):
            ret = 'DateTime'
        elif isinstance(typeObj, model.DurationType):
            ret = 'Duration'
        elif isinstance(typeObj, model.DictionaryType):
            ret = 'HashMap<String, {}>'.format(printRustType(typeObj.valueType, False, True))
        elif isinstance(typeObj, model.ComplexType):
            ret = typeObj.name
        else:
            ret = '???'

        if isArray:
            tmpStr = ""
            for i in range(arrayDimensions):
                tmpStr = tmpStr + "Vec<"
            tmpStr = tmpStr + ret
            for i in range(arrayDimensions):
                tmpStr = tmpStr + ">"
            ret = tmpStr
        if (not isRequired):
            ret = "Option<{}>".format(ret)
        return ret

    def getEnumDefaultValue(type):
        if type.default is not None:
            return secureEnumValues(type.default)
        else:
            return secureEnumValues(type.values[0])

    def secureEnumValues(value):
        pattern = re.compile("^[0-9]")
        return '_' + value if pattern.match(value) else value

    def isEnumDefaultValue(value, type):
        return getEnumDefaultValue(type) == secureEnumValues(value)


%>// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: ${templateFile} v${templateVersion})

use serde_json;
use serde::Deserialize;
% if modelFuncs.isUuidContained(modelTypes):
use uuid::Uuid;
% endif
% if modelFuncs.isTypeContained(modelTypes, model.DictionaryType):
use std::collections::HashMap;
% endif
%if modelFuncs.isTypeContained(modelTypes, model.DateType):
use chrono::Date
%endif
%if modelFuncs.isTypeContained(modelTypes, model.DateTimeType):
use chrono::DateTime
%endif
%if modelFuncs.isTypeContained(modelTypes, model.DurationType):
use chrono::Duration
%endif

% for type in modelTypes:
    % if modelFuncs.isEnumType(type):
        % if type.description != None:
/* ${templateHelper.addLineBreakToDescription(type.description,4)}
*/
        % endif
pub enum ${type.name} {
        % for value in type.values:
    ${stringUtils.toUpperCamelCase(value)},
        % endfor
}

impl ${type.name} {
    fn as_str(&self) -> &'static str {
        match *self {
        % for value in type.values:
            ${type.name}::${stringUtils.toUpperCamelCase(value)} => "${value}",
        % endfor
        }
    }

    fn from_str<'a>(s:&'a str) -> Result<${type.name}, &'static str> {
        match s {
        % for value in type.values:
            "${value}" => Ok(${type.name}::${stringUtils.toUpperCamelCase(value)}),
        % endfor
            _ => Err("unknown value"),
        }
    }

}

    % endif

    % if hasattr(type, "properties"):
        % if type.description != None:
/* ${templateHelper.addLineBreakToDescription(type.description,4)}
*/
        % endif
pub struct ${type.name} {
        % for property in type.properties:

            % if property.description != None:
    // ${property.description}
            % endif
    pub ${stringUtils.toSnakeCase(property.name)}: ${printRustType(property.type, property.isArray, property.required, property.arrayDimensions)},
        % endfor
}

impl ${type.name} {
    pub fn new (
        % for property in type.properties:
            % if (property.required) and (not property.isArray):
        ${stringUtils.toSnakeCase(property.name)}: ${printRustType(property.type, property.isArray, property.required, property.arrayDimensions)},
            % endif
        % endfor
    ) -> Self {
        Self {
        % for property in type.properties:
            % if (property.required):
                % if property.isArray:
        ${stringUtils.toSnakeCase(property.name)}: Vec::new(),
                % elif isinstance(property.type, model.DictionaryType):
        ${stringUtils.toSnakeCase(property.name)}: HashMap::new(),
                % else:
        ${stringUtils.toSnakeCase(property.name)}: ${stringUtils.toSnakeCase(property.name)},
                % endif
            % else:
        ${stringUtils.toSnakeCase(property.name)}: None,
            % endif
        % endfor
        }
    }
}

    % endif

% endfor

% if testMod is not None:
#[cfg(test)]
    % if testModPath is not None:
#[path = "${testModPath}"]
    % endif
mod ${testMod};
% endif
