from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class OtpSeed(BaseModel):
    class Meta:
        name = "otp_seed"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
