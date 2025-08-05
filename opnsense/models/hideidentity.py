from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Hideidentity(BaseModel):
    class Meta:
        name = "hideidentity"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
