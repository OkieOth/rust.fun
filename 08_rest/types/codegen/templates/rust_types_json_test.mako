<%
    import re
    import yacg.model.model as model
    import yacg.templateHelper as templateHelper
    import yacg.model.modelFuncs as modelFuncs
    import yacg.util.stringUtils as stringUtils

    templateFile = 'rust_types_json_test.mako'
    templateVersion = '1.1.0'

    modelMod = templateParameters.get('modelMod','<<PLEASE SET modelMod TEMPLATE PARAM>>')

    def getTestValue(property):
        typeObj = property.type
        if typeObj is None:
            return f'??? // no type for prop={property.name}'
        elif isinstance(typeObj, model.IntegerType):
            if typeObj.format is None or typeObj.format == model.IntegerTypeFormatEnum.INT32:
                return '42'
            else:
                return '42'
        elif isinstance(typeObj, model.ObjectType):
            return '{}'
        elif isinstance(typeObj, model.NumberType):
            if typeObj.format is None or typeObj.format != model.NumberTypeFormatEnum.DOUBLE:
                return '0.42'
            else:
                return '0.42'
        elif isinstance(typeObj, model.BooleanType):
            return 'true'
        elif isinstance(typeObj, model.StringType):
            return '"dummy".to_string()'
        elif isinstance(typeObj, model.BytesType):
            return '42'
        elif isinstance(typeObj, model.UuidType):
            return 'Uuid::nil()'
        elif isinstance(typeObj, model.EnumType):
            return f'{typeObj.name}::{stringUtils.toUpperCamelCase(typeObj.values[0])}'
        elif isinstance(typeObj, model.DateType):
            return 'Utc::today()'
        elif isinstance(typeObj, model.TimeType):
            return '"12:05"'
        elif isinstance(typeObj, model.DateTimeType):
            return 'Utc::now()'
        elif isinstance(typeObj, model.DurationType):
            return 'Duration::minutes(42)'
        else:
            return f'??? // unhandled type for prop={property.name}, type={property.type.name}'

    def printCommaIfNeeded(i, requiredPropList):
        if i+1 < len(requiredPropList):
            return ","
        else:
            return ""

%>// Attention, this file is generated. Manual changes get lost with the next
// run of the code generation.
// created by yacg (template: ${templateFile} v${templateVersion})

% if modelFuncs.isUuidContained(modelTypes):
use uuid::Uuid;
% endif
% if modelFuncs.isTypeContained(modelTypes, model.DictionaryType):
use std::collections::HashMap;
% endif
%if modelFuncs.isTypeContained(modelTypes, model.DateType) or modelFuncs.isTypeContained(modelTypes, model.DateTimeType):
use chrono::Utc;
%endif
%if modelFuncs.isTypeContained(modelTypes, model.DateType):
use chrono::Date
%endif
%if modelFuncs.isTypeContained(modelTypes, model.DateTimeType):
use chrono::DateTime
%endif
%if modelFuncs.isTypeContained(modelTypes, model.DurationType):
use chrono::Duration
%endif
use crate::${modelMod};
use serde_json;



% for type in modelTypes:
    % if hasattr(type, "properties"):
<% 
    requiredPropList = modelFuncs.getRequiredProperties(type) 
%>
#[test]
fn test_json_${stringUtils.toSnakeCase(type.name)}() {
        % for i in range(len(requiredPropList)):
    let ${stringUtils.toSnakeCase(requiredPropList[i].name)} = ${getTestValue(requiredPropList[i])};
        % endfor
    let first = ${modelMod}::${type.name}::new(
        % for i in range(len(requiredPropList)):
        ${stringUtils.toSnakeCase(requiredPropList[i].name)}${printCommaIfNeeded(i, requiredPropList)}
        % endfor
    );
    let json_string = serde_json::to_string(&first).expect("Failed to serialize object to JSON");
    let second: ${modelMod}::${type.name} =
        serde_json::from_str(&json_string).expect("Failed to deserialize JSON to object");
    assert_eq!(first, second);
}
    % endif

% endfor
