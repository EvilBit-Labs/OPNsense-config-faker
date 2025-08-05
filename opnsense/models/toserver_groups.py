from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class ToserverGroups(BaseModel):
    class Meta:
        name = "toserver_groups"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
