from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.default_action import DefaultAction

__NAMESPACE__ = "https://opnsense.org/config"


class Acls(BaseModel):
    class Meta:
        name = "acls"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    default_action: DefaultAction = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
