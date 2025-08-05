from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.profile import Profile
from opnsense.models.toclient_groups import ToclientGroups
from opnsense.models.toserver_groups import ToserverGroups

__NAMESPACE__ = "https://opnsense.org/config"


class Detect(BaseModel):
    class Meta:
        name = "detect"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    profile: Profile = field(
        metadata={
            "name": "Profile",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    toclient_groups: ToclientGroups = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    toserver_groups: ToserverGroups = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
