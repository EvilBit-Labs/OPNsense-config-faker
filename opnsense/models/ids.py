from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.file_tags import FileTags
from opnsense.models.files import Files
from opnsense.models.general import General
from opnsense.models.policies import Policies
from opnsense.models.rules import Rules
from opnsense.models.user_defined_rules import UserDefinedRules

__NAMESPACE__ = "https://opnsense.org/config"


class Ids(BaseModel):
    class Meta:
        name = "IDS"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    rules: Rules = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    policies: Policies = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    user_defined_rules: UserDefinedRules = field(
        metadata={
            "name": "userDefinedRules",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    files: Files = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    file_tags: FileTags = field(
        metadata={
            "name": "fileTags",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
