from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.group import Group

__NAMESPACE__ = "https://opnsense.org/config"


class Ssh(BaseModel):
    class Meta:
        name = "ssh"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    group: Group = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
