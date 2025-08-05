from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.route import Route

__NAMESPACE__ = "https://opnsense.org/config"


class Staticroutes(BaseModel):
    class Meta:
        name = "staticroutes"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    route: Route = field(
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
