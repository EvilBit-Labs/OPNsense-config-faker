from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class EnableWpad(BaseModel):
    class Meta:
        name = "enable_wpad"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
