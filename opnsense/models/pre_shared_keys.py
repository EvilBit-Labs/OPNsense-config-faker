from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class PreSharedKeys(BaseModel):
    class Meta:
        name = "preSharedKeys"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
