from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class ToclientGroups(BaseModel):
    class Meta:
        name = "toclient_groups"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
